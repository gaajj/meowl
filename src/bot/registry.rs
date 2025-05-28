use crate::{bot::{Data, Error}, commands::*};
use poise::Command;

pub fn all_commands() -> Vec<Command<Data, Error>> {
    vec![
        help(),
        commands(),
    ]
}
