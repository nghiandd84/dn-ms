use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Default)]
pub struct QueryParams {
    #[serde(default, deserialize_with = "deserialize_comma_separated")]
    includes: Vec<String>,
}

fn deserialize_comma_separated<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.split(',').map(|s| s.to_string()).collect())
}

impl QueryParams {
    pub fn includes(&self) -> Vec<String> {
        self.includes.clone()
    }
}

