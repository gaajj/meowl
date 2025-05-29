use crate::bot::{Data, Error};
use crate::commands;
use poise::Command;

macro_rules! collect_commands {
    ($($path:path),* $(,)?) => {
        vec![
            $($path()),*
        ]
    };
}

pub fn all_commands() -> Vec<Command<Data, Error>> {
    collect_commands![
        commands::help::help,
        commands::commands::commands,
        commands::qr::qr,
    ]
}
