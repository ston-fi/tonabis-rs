#!/usr/bin/env python3
"""Compile and run all umbrella feature selections from outside the workspace."""
import itertools
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
PROTOCOLS = ('dedust', 'moon', 'stonks', 'swap_coffee', 'tonco')


def run(arguments, directory, environment):
    result = subprocess.run(arguments, cwd=directory, env=environment, text=True, capture_output=True)
    if result.returncode:
        raise RuntimeError(' '.join(arguments) + '\n' + result.stdout + result.stderr)
    return result.stdout


def main():
    environment = dict(os.environ)
    environment['CARGO_TARGET_DIR'] = str(ROOT / 'target/integration-consumer')
    with tempfile.TemporaryDirectory(prefix='tonabis-consumer-') as scratch:
        directory = Path(scratch)
        (directory / 'src').mkdir()
        shutil.copyfile(Path(__file__).with_name('consumer.rs'), directory / 'src/main.rs')
        shutil.copyfile(ROOT / 'Cargo.lock', directory / 'Cargo.lock')
        metadata = json.loads(run(['cargo', 'metadata', '--locked', '--offline', '--format-version', '1'], ROOT, environment))
        ton_packages = {package['name']: package for package in metadata['packages']
                        if package['name'] in ('ton', 'ton_core', 'ton_macros')}
        for package in ton_packages.values():
            if package['source'] != 'registry+https://github.com/rust-lang/crates.io-index':
                raise RuntimeError(f"{package['name']} must resolve from crates.io")
        ton_version = ton_packages['ton']['version']
        features = '\n'.join(f'{name} = ["tonabis/{name}", "dep:ton"]' for name in PROTOCOLS)
        (directory / 'Cargo.toml').write_text(
            '[package]\nname = "integration_consumer"\nversion = "0.0.0"\nedition = "2024"\n'
            '[dependencies]\ntonabis = { path = ' + json.dumps(str(ROOT / 'crates/tonabis')) + ' }\n'
            f'ton = {{ version = "={ton_version}", optional = true }}\n'
            '[features]\ndefault = []\n' + features + '\n'
            'all = ["tonabis/all", "dep:ton"]\n[profile.dev]\ndebug = false\n'
        )
        # Adjust only the temporary consumer lockfile, preserving workspace registry pins.
        run(['cargo', 'run', '--offline'], directory, environment)
        selections = [list(subset) for size in range(6) for subset in itertools.combinations(PROTOCOLS, size)]
        selections += [['all']]
        for selected in selections:
            flags = ['--no-default-features']
            if selected:
                flags += ['--features', ','.join(selected)]
            run(['cargo', 'run', '--locked', '--offline', *flags], directory, environment)
            tree = run(['cargo', 'tree', '--locked', '--offline', '--prefix', 'none', '--edges', 'normal', *flags], directory, environment)
            actual = set(re.findall(r'^tonabis_(\w+) v', tree, re.MULTILINE))
            expected = {'core'} | (set(PROTOCOLS) if selected == ['all'] else set(selected))
            if actual != expected:
                raise RuntimeError(f'{selected}: expected {expected}, dependency graph has {actual}\n{tree}')
            print('PASS ' + (','.join(selected) or 'no-default-features'), flush=True)
        run(['cargo', 'run', '--locked', '--offline'], directory, environment)
        tree = run(['cargo', 'tree', '--locked', '--offline', '--prefix', 'none', '--edges', 'normal'], directory, environment)
        if set(re.findall(r'^tonabis_(\w+) v', tree, re.MULTILINE)) != {'core'}:
            raise RuntimeError('Default graph unexpectedly includes protocol crates:\n' + tree)
        print('PASS default: core only; 32 subsets plus all and default passed', flush=True)


if __name__ == '__main__':
    main()
