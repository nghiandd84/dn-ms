use serde::{Deserialize, Deserializer};

/// Represents a parsed include parameter with optional field selection.
/// e.g., `client[id,name]` → IncludeParam { name: "client", fields: Some(["id", "name"]) }
/// e.g., `permissions` → IncludeParam { name: "permissions", fields: None }
#[derive(Debug, Clone)]
pub struct IncludeParam {
    pub name: String,
    pub fields: Option<Vec<String>>,
}

#[derive(Deserialize, Default, Debug)]
pub struct QueryParams {
    #[serde(default, deserialize_with = "deserialize_includes")]
    includes: Vec<IncludeParam>,

    #[serde(default, deserialize_with = "deserialize_fields")]
    fields: Vec<String>,
}

/// Splits on commas that are NOT inside brackets, then parses each segment.
fn deserialize_includes<'de, D>(deserializer: D) -> Result<Vec<IncludeParam>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(parse_includes(&s))
}

/// Deserialize a comma-separated list of field names.
fn deserialize_fields<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
}

/// Parse an includes string like `permissions,client[id,name]` into IncludeParams.
fn parse_includes(s: &str) -> Vec<IncludeParam> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, ch) in s.char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' if depth == 0 => {
                let segment = &s[start..i];
                if !segment.is_empty() {
                    result.push(parse_include_segment(segment));
                }
                start = i + 1;
            }
            _ => {}
        }
    }
    // last segment
    let segment = &s[start..];
    if !segment.is_empty() {
        result.push(parse_include_segment(segment));
    }
    result
}

/// Parse a single segment like `client[id,name]` or `permissions`.
fn parse_include_segment(segment: &str) -> IncludeParam {
    if let Some(bracket_start) = segment.find('[') {
        let name = segment[..bracket_start].trim().to_string();
        let fields_str = segment[bracket_start + 1..].trim_end_matches(']');
        let fields: Vec<String> = fields_str.split(',').map(|f| f.trim().to_string()).collect();
        IncludeParam {
            name,
            fields: Some(fields),
        }
    } else {
        IncludeParam {
            name: segment.trim().to_string(),
            fields: None,
        }
    }
}

impl QueryParams {
    /// Returns just the relation names (backward compatible with the Query macro).
    /// `client[id,name]` → "client", `permissions` → "permissions"
    pub fn includes(&self) -> Vec<String> {
        self.includes.iter().map(|p| p.name.clone()).collect()
    }

    /// Returns the full include params with optional field selections.
    pub fn include_params(&self) -> &Vec<IncludeParam> {
        &self.includes
    }

    /// Get the field selection for a specific include, if any.
    /// Returns None if the include doesn't exist or has no field selection.
    pub fn include_fields(&self, name: &str) -> Option<&Vec<String>> {
        self.includes
            .iter()
            .find(|p| p.name == name)
            .and_then(|p| p.fields.as_ref())
    }

    /// Returns the top-level field selection.
    /// Empty vec means no filtering (return all fields).
    pub fn fields(&self) -> &Vec<String> {
        &self.fields
    }

    pub fn add_includes(&mut self, extra: Vec<String>) {
        for inc in extra {
            if !self.includes.iter().any(|p| p.name == inc) {
                self.includes.push(IncludeParam {
                    name: inc,
                    fields: None,
                });
            }
        }
    }
}
