mod ban;
mod delete_message;
mod kick;
mod timeout;
mod unban;

use crate::discord::*;

pub fn modal_responders() -> Vec<Box<dyn ModalHandler + Send + Sync>> {
    let responders: Vec<Box<dyn ModalHandler + Send + Sync>> = vec![
        Box::new(delete_message::DeleteMessage),
        Box::new(timeout::Timeout),
        Box::new(kick::Kick),
        Box::new(ban::Ban),
    ];

    responders
}
