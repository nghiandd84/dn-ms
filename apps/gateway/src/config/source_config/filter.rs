use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Filter {
    pub name: String,
    pub path: Option<PathFilter>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "operator")]
pub enum PathFilter {
    #[serde(rename = "start_with")]
    StartWith { value: String },
    #[serde(rename = "end_with")]
    EndWith { value: String },
}
