use serde::{Deserialize, Serialize};

fn hex_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    u32::from_str_radix(s.trim_start_matches("#"), 16).map_err(serde::de::Error::custom)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Constants {
    pub colors: Colors,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename = "colors")]
pub struct Colors {
    #[serde(rename = "default")]
    #[serde(deserialize_with = "hex_to_u32")]
    pub default: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub primary: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub secondary: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub success: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub danger: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub warning: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub azoxo: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub green: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub yellow: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub fuchsia: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub magic: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub developer: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub balance: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub brilliance: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub nitro: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub bravery: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub royal: u32,
}
