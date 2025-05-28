use crate::bot::{Data, Error};
use poise::serenity_prelude as serenity;
use tracing::info;

pub async fn on_event(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { .. } => {
            info!("Received Ready event.");
        }
        _ => {}
    }
    Ok(())
}
