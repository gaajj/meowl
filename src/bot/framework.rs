use crate::bot::{error::BotError, events::on_event, registry::all_commands, Data, Error};
use poise::serenity_prelude::GuildId;
use std::{sync::Arc, time::Duration};
use tracing::{error, info};

pub async fn create_framework(guild_id: GuildId, prefix: String) -> poise::Framework<Data, Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: all_commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(prefix),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(Duration::from_secs(3600)))),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            pre_command: |ctx| Box::pin(async move {
                info!("Executing command: {}...", ctx.command().qualified_name);
            }),
            post_command: |ctx| Box::pin(async move {
                info!("Finished command: {}.", ctx.command().qualified_name);
            }),
            skip_checks_for_owners: false,
            event_handler: |ctx, event, fw, data| Box::pin(on_event(ctx, event, fw, data)),
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                info!("Bot is ready! Logged in as: {}", ready.user.name);
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await
                    .map_err(|e| BotError::Other(anyhow::anyhow!("Registering commands failed: {}", e)))?;
                Ok(Data {})
            })
        })
        .build()
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!("Error in command `{}`: {}", ctx.command().name, error);
        }
        poise::FrameworkError::Setup { error, .. } => {
            error!("Error during setup: {}", error);
        }
        other => {
            if let Err(e) = poise::builtins::on_error(other).await {
                error!("Error handling framework error: {}", e);
            }
        }
    }
}
