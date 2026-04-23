use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct QueryParams {
    // We use a helper function to split the string
    #[serde(deserialize_with = "deserialize_comma_separated")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_comma_separated_includes() {
        let json = r#"{"includes":"wallet,transaction"}"#;
        let params: QueryParams = serde_json::from_str(json).unwrap();

        assert_eq!(
            params.includes(),
            vec!["wallet".to_string(), "transaction".to_string()]
        );
    }

    #[test]
    fn deserialize_single_include() {
        let json = r#"{"includes":"wallet"}"#;
        let params: QueryParams = serde_json::from_str(json).unwrap();

        assert_eq!(params.includes(), vec!["wallet".to_string()]);
    }

    #[test]
    fn deserialize_empty_include_list() {
        let json = r#"{"includes":""}"#;
        let params: QueryParams = serde_json::from_str(json).unwrap();

        assert_eq!(params.includes(), vec!["".to_string()]);
    }
}
