use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Guild {
    pub channels: Channels,

    pub roles: Roles,
}

fn str_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize)]
pub struct Channels {
    #[serde(deserialize_with = "str_to_u64")]
    pub announcement: u64,
    #[serde(deserialize_with = "str_to_u64")]
    pub mousetrap: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Roles {
    #[serde(deserialize_with = "str_to_u64")]
    pub apps: u64,
    #[serde(deserialize_with = "str_to_u64")]
    pub kernel: u64,
    #[serde(deserialize_with = "str_to_u64")]
    pub stf: u64,
    #[serde(deserialize_with = "str_to_u64")]
    pub boosters: u64,
}