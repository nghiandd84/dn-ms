use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, ToSchema)]
#[schema(examples(1, -1) )]
pub enum OrderDirection {
    Asc = 1,
    Desc = -1,
}

impl<'de> Deserialize<'de> for OrderDirection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        // Accept either an integer (1 / -1) or a string ("asc" / "desc").
        // Query-string values arrive as strings, so also handle "1" / "-1".
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Raw {
            Int(i32),
            Str(String),
        }

        match Raw::deserialize(deserializer)? {
            Raw::Int(1) => Ok(OrderDirection::Asc),
            Raw::Int(-1) => Ok(OrderDirection::Desc),
            Raw::Int(_) => Err(D::Error::custom("Invalid order direction value")),
            Raw::Str(s) => match s.trim().to_ascii_lowercase().as_str() {
                "asc" | "1" => Ok(OrderDirection::Asc),
                "desc" | "-1" => Ok(OrderDirection::Desc),
                _ => Err(D::Error::custom("Invalid order direction value")),
            },
        }
    }
}

#[derive(Deserialize, Debug, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Order {
    #[serde(default = "default_order_name")]
    #[param(value_type = Option<String>)]
    pub order_name: Option<String>,
    #[serde(default = "default_order_direction")]
    #[param(inline, value_type = Option<String>)]
    pub order_direction: Option<OrderDirection>,
}

impl Order {
    pub fn default() -> Self {
        Self {
            order_name: None,
            order_direction: None,
        }
    }
}

fn default_order_name() -> Option<String> {
    None
}

fn default_order_direction() -> Option<OrderDirection> {
    None
}
