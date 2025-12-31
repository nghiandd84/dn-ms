use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Languages {
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "vi-VN")]
    ViVn,
}

impl Default for Languages {
    fn default() -> Self {
        Languages::EnUs
    }
}

impl Languages {
    pub fn as_str(&self) -> &str {
        match self {
            Languages::EnUs => "en-US",
            Languages::ViVn => "vi-VN",
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Languages::EnUs => b"en-US",
            Languages::ViVn => b"vi-VN",
        }
    }
}

pub fn extract_language(accept_language: &str) -> Languages {
    if accept_language.to_lowercase().contains("vi") {
        Languages::ViVn
    } else {
        Languages::EnUs
    }
}
