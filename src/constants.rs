use config::{Config, File, FileFormat};
use crate::models::*;
use once_cell::sync::Lazy;

// Avatar
pub const IMG_BANGBOO: &[u8] = include_bytes!("../assets/avatar/Bangboo.png");
pub const IMG_OFFICER: &[u8] = include_bytes!("../assets/avatar/Officer.png");

// Member
pub const IMG_DEFAULT_AVATAR: &[u8] = include_bytes!("../assets/member/default_avatar.png");

// Fonts
pub const FONT_FREDOKA: &[u8] = include_bytes!("../assets/fonts/Fredoka-Medium.ttf");
pub const FONT_ROBOTO: &[u8] = include_bytes!("../assets/fonts/Roboto-Medium.ttf");

// Cards
//pub const CARD_BACK: &[u8] = include_bytes!("../assets/cards/card-back.png");
pub const CARD_LEFT: &[u8] = include_bytes!("../assets/cards/card-left.png");
pub const CARD_MOD: &[u8] = include_bytes!("../assets/cards/card-mod.png");
pub const CARD_NEW: &[u8] = include_bytes!("../assets/cards/card-new.png");

// Discloud
const DISCLOUDCONFIG: &str = include_str!("../discloud.config");
pub static APPID: Lazy<String> = Lazy::new(|| {
    let discloud = match Config::builder()
        .add_source(File::from_str(DISCLOUDCONFIG, FileFormat::Ini))
        .build()
    {
        Ok(config) => config,
        Err(_) => return String::new(),
    };
    discloud.get("ID").unwrap_or_default()
});

// JSON
const COLORSJSON: &str = include_str!("../data/colors.json");
pub static COLORS: Lazy<Colors> = Lazy::new(|| {
    let constants: Constants = serde_json::from_str(&COLORSJSON).unwrap();
    constants.colors
});

const EMOJISJSON: &str = include_str!("../data/emojis.json");
pub static EMOJIS: Lazy<Emojis> = Lazy::new(|| serde_json::from_str(&EMOJISJSON).unwrap());

const FABJSON: &str = include_str!("../data/fab.json");
pub static FAB: Lazy<Fab> = Lazy::new(|| serde_json::from_str(&FABJSON).unwrap());

const GUILDJSON: &str = include_str!("../data/guild.json");
pub static GUILD: Lazy<Guild> = Lazy::new(|| serde_json::from_str(&GUILDJSON).unwrap());


