use crate::discord::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use twilight_gateway::EventType;

pub struct Handler {
    pub event_handlers: HashMap<EventType, Box<dyn EventHandler + Send + Sync>>,
    pub prefix_command_handlers: HashMap<String, Box<dyn PrefixCommandHandler + Send + Sync>>,
    pub app_command_handlers: HashMap<String, Box<dyn SlashCommandHandler + Send + Sync>>,
    pub modal_handlers: HashMap<String, Box<dyn ModalHandler + Send + Sync>>,
    pub message_component_handlers: HashMap<String, Box<dyn MessageComponentHandler + Send + Sync>>,
}

pub static HANDLERS: Lazy<Handler> = Lazy::new(|| {
    let events = events();
    let mut event_handlers = HashMap::new();
    for event in events {
        let kind = event.event();
        event_handlers.insert(kind, event);
    }

    let slash_commands = slash_commands();
    //let mut commands = Vec::new();
    let mut slash_command_handlers = HashMap::new();
    for slash_command in slash_commands {
        let name = slash_command.command().name.clone();
        //commands.push(cmd);
        slash_command_handlers.insert(name, slash_command);
    }

    let prefix_commands = prefix_commands();
    let mut prefix_command_handlers = HashMap::new();
    for command in prefix_commands {
        let name = format!("!{}", command.name());
        prefix_command_handlers.insert(name, command);
    }

    let modals = modal_responders();
    let mut modal_handlers = HashMap::new();
    for modal in modals {
        let custom_id = modal.custom_id();
        modal_handlers.insert(custom_id, modal);
    }
    
    let message_components = message_component_responders();
    let mut message_component_handlers = HashMap::new();
    for msg_component in message_components {
        let custom_id = msg_component.custom_id();
        message_component_handlers.insert(custom_id, msg_component);
    }

    Handler {
        prefix_command_handlers,
        event_handlers,
        app_command_handlers: slash_command_handlers,
        modal_handlers,
        message_component_handlers
    }
});
