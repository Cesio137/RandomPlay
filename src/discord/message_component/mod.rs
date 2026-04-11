mod discloud;

use crate::discord::*;

pub fn message_component_responders() -> Vec<Box<dyn MessageComponentHandler + Send + Sync>> {
    let responders: Vec<Box<dyn MessageComponentHandler + Send + Sync>> = vec![
        Box::new(discloud::Status),
        Box::new(discloud::Logs),
    ];

    responders
}