//! Executable BASTION semantic-delivery program.

use std::collections::{BTreeMap, BTreeSet};

pub mod sem003;
pub mod sem004;
pub mod sem005;
pub mod sem006;
pub mod sem007;
pub mod sem008;
pub mod sem009;
pub mod sem010;
pub mod sem011;
pub mod sem012;

const MAX_INPUT_BYTES: usize = 128 * 1024;

#[derive(Debug)]
pub struct Document {
    fields: BTreeMap<String, String>,
}

impl Document {
    /// Parses a bounded key-value document.
    ///
    /// # Errors
    ///
    /// Rejects oversized, malformed, or duplicate fields.
    pub fn parse(input: &str) -> Result<Self, String> {
        if input.len() > MAX_INPUT_BYTES {
            return Err("input exceeds 131072 bytes".into());
        }
        let mut fields = BTreeMap::new();
        for (index, line) in input.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| format!("line {} lacks '='", index + 1))?;
            if fields.insert(key.to_owned(), value.to_owned()).is_some() {
                return Err(format!("duplicate field: {key}"));
            }
        }
        Ok(Self { fields })
    }

    /// Requires an exact closed set of keys.
    ///
    /// # Errors
    ///
    /// Rejects missing or unknown keys.
    pub fn exact(&self, keys: &[&str]) -> Result<(), String> {
        let expected: BTreeSet<&str> = keys.iter().copied().collect();
        if let Some(key) = keys.iter().find(|key| !self.fields.contains_key(**key)) {
            return Err(format!("missing field: {key}"));
        }
        if let Some(key) = self
            .fields
            .keys()
            .find(|key| !expected.contains(key.as_str()))
        {
            return Err(format!("unknown field rejected: {key}"));
        }
        Ok(())
    }

    /// Returns a required safe text value.
    ///
    /// # Errors
    ///
    /// Rejects missing or unsafe text.
    pub fn text(&self, key: &str) -> Result<&str, String> {
        let value = self
            .fields
            .get(key)
            .ok_or_else(|| format!("missing field: {key}"))?;
        if value.is_empty()
            || value.len() > 512
            || value
                .chars()
                .any(|character| character.is_control() || character == '"')
        {
            return Err(format!("unsafe text field: {key}"));
        }
        Ok(value)
    }

    /// Returns a required unsigned integer.
    ///
    /// # Errors
    ///
    /// Rejects a missing or invalid integer.
    pub fn u64(&self, key: &str) -> Result<u64, String> {
        self.text(key)?
            .parse::<u64>()
            .map_err(|_| format!("invalid integer: {key}"))
    }

    /// Returns a required signed integer.
    ///
    /// # Errors
    ///
    /// Rejects a missing or invalid integer.
    pub fn i64(&self, key: &str) -> Result<i64, String> {
        self.text(key)?
            .parse::<i64>()
            .map_err(|_| format!("invalid signed integer: {key}"))
    }
}

/// Runs one named semantic delivery.
///
/// # Errors
///
/// Returns a closed parsing or semantic validation error.
pub fn run(delivery: &str, input: &str) -> Result<String, String> {
    match delivery {
        "sem-003" => sem003::run(input),
        "sem-004" => sem004::run(input),
        "sem-005" => sem005::run(input),
        "sem-006" => sem006::run(input),
        "sem-007" => sem007::run(input),
        "sem-008" => sem008::run(input),
        "sem-009" => sem009::run(input),
        "sem-010" => sem010::run(input),
        "sem-011" => sem011::run(input),
        "sem-012" => sem012::run(input),
        _ => Err(format!("unknown delivery: {delivery}")),
    }
}
