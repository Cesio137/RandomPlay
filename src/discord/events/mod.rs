mod ban_add;
mod member_add;
mod member_remove;
mod member_update;
mod message_create;
mod ready;

use super::base::*;

pub fn events() -> Vec<Box<dyn EventHandler + Send + Sync>> {
    let events: Vec<Box<dyn EventHandler + Send + Sync>> = vec![
        Box::new(ban_add::BanAdd),
        Box::new(member_add::MemberAdded),
        Box::new(member_remove::MemberRemove),
        Box::new(member_update::MemberUpdate),
        Box::new(message_create::MessageCreate),
        Box::new(ready::Ready),
    ];

    events
}
