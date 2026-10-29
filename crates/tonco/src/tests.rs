mod test_boundaries;
mod test_getter_provider;
mod test_getters;
mod test_suffixes;
mod test_wire;
// Counts are part of the pinned reference corpus, not results calculated by
// the implementation under test. Empty or partially captured corpora must fail.
fn fixture_rows<'a>(
    value: &'a serde_json::Value,
    expected: usize,
    name: &str,
) -> anyhow::Result<&'a [serde_json::Value]> {
    let rows = value.as_array().ok_or_else(|| anyhow::anyhow!("{name} is not an array"))?;
    anyhow::ensure!(
        rows.len() == expected,
        "{name} corpus incomplete: expected {expected}, got {}",
        rows.len()
    );
    Ok(rows)
}
