use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Fab {
    pub engine_user_setings: ProductInfos,

    pub internet_protocol: ProductInfos,
}

fn str_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize)]
pub struct ProductInfos {
    pub product_name: String,

    pub product_desc: String,

    pub thumb_link: String,

    pub product_url: String,

    pub doc_url: String,
}