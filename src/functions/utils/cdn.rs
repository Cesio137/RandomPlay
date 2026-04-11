use twilight_model::user::User;

pub fn display_avatar_url(user: &User, size: u16) -> Option<String> {
    let id = &user.id.get();
    let Some(hash) = user.avatar else {
        return None;
    };
    let image_hash = hash.to_string();
    if size == 0 {
        Some(format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png",
            id, image_hash
        ))
    } else {
        Some(format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png?size={}",
            id, image_hash, size
        ))
    }
}
