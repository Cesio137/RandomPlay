mod age;
mod moderate;
mod discloud;
mod prompt;
mod fab;
mod social;

use crate::discord::*;

pub fn prefix_commands() -> Vec<Box<dyn PrefixCommandHandler + Send + Sync>> {
    let commands: Vec<Box<dyn PrefixCommandHandler + Send + Sync>> = vec![
        //Box::new(prefix::canvas::Canvas),
    ];

    commands
}

pub fn slash_commands() -> Vec<Box<dyn SlashCommandHandler + Send + Sync>> {
    let commands: Vec<Box<dyn SlashCommandHandler + Send + Sync>> =
        vec![
            Box::new(age::Age),
            Box::new(discloud::Discloud),
            Box::new(fab::Fab),
            Box::new(moderate::Moderate),
            Box::new(prompt::Prompt),
            Box::new(social::Social),
        ];

    commands
}
