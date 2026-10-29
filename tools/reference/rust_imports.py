"""Use unambiguous type imports in generated Rust; preserve functions and collisions."""
import re

USE = re.compile(r'(?m)^[ \t]*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);')
PATH = re.compile(r'\b(?:ton|ton_core|tonabis_core|crate|std|fastnum)::(?:[a-z_][a-z_0-9]*::)*[A-Z][A-Za-z_0-9]*')


def code_mask(source):
    """Mask Rust literals and comments without changing offsets or line breaks.

    This is a lexical guard for generated declarations, not a Rust parser.
    Lifetimes remain code; nested block comments and raw string delimiters matter.
    """
    masked = list(source)
    token = re.compile(r'//|/\*|(?:br|cr|r)(\#*)"|"|\'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^\'\\\n])\'')
    offset = 0
    while match := token.search(source, offset):
        start = match.start()
        end = match.end()
        if match[0] == '//':
            newline = source.find('\n', end)
            end = len(source) if newline == -1 else newline
        elif match[0] == '/*':
            depth = 1
            while depth and end < len(source):
                if source.startswith('/*', end):
                    depth += 1
                    end += 2
                elif source.startswith('*/', end):
                    depth -= 1
                    end += 2
                else:
                    end += 1
        elif match[1] is not None:
            delimiter = '"' + match[1]
            closing = source.find(delimiter, end)
            end = len(source) if closing == -1 else closing + len(delimiter)
        elif match[0] == '"':
            while end < len(source):
                char = source[end]
                end += 1
                if char == '\\':
                    end += 1
                elif char == '"':
                    break
            end = min(end, len(source))
        for index in range(start, end):
            if source[index] != '\n':
                masked[index] = ' '
        offset = end
    return ''.join(masked)


def _imports(tree, prefix=''):
    """Expand Rust's grouped use syntax, including explicit aliases."""
    depth = 0
    start = 0
    entries = []
    for i, char in enumerate(tree + ','):
        if char == '{':
            depth += 1
        elif char == '}':
            depth -= 1
        elif char == ',' and depth == 0:
            entries.append(tree[start:i].strip())
            start = i + 1
    for entry in entries:
        if not entry:
            continue
        if '{' in entry:
            head, children = entry.split('{', 1)
            yield from _imports(children.rsplit('}', 1)[0], prefix + head.strip())
        else:
            parts = re.split(r'\s+as\s+', entry)
            path = prefix + parts[0].strip()
            yield (parts[-1].strip() if len(parts) > 1 else path.rsplit('::', 1)[-1], path)


def shorten_type_paths(source):
    """Shorten full type paths only when their local name has one owner.

    Keep same-name macros/traits qualified and leave wildcard-import scopes alone.
    This operates on the explicit declarations emitted by these generators.
    """
    code = code_mask(source)
    uses = list(USE.finditer(code))
    imported = {}
    global_names = set()
    for match in uses:
        for name, path in _imports(match[1]):
            imported.setdefault(name, set()).add(path)
            if not match[0][0].isspace():
                global_names.add(name)
    if '*' in imported:
        return source
    body = USE.sub('', code)
    declared = set(re.findall(r'\b(?:struct|enum|trait|type)\s+(\w+)', body))
    candidates = {}
    for path in PATH.findall(body):
        name = path.rsplit('::', 1)[-1]
        candidates.setdefault(name, set()).add(path)
    replacements = {}
    additions = []
    for name, paths in candidates.items():
        if name in declared or name in ('TLB', 'FromTVMStack', 'ToTVMStack'):
            continue
        if len(paths | imported.get(name, set())) != 1:
            continue
        path = next(iter(paths))
        replacements[path] = name
        if name not in global_names:
            additions.append('use ' + path + ';\n')
    # Apply only code-token matches, backwards so offsets remain valid.
    for match in reversed(list(PATH.finditer(code))):
        if not any(use.start() <= match.start() < use.end() for use in uses):
            source = source[:match.start()] + replacements.get(match[0], match[0]) + source[match.end():]
    lines = source.splitlines(True)
    i = 0
    while i < len(lines) and (lines[i].startswith(('//!', '#![')) or not lines[i].strip()):
        i += 1
    lines[i:i] = sorted(additions)
    return ''.join(lines)
