use crate::bot::{Context, Error};
use image::{ImageFormat, Luma};
use qrcode::QrCode;
use std::io::Cursor;
use poise::serenity_prelude::CreateAttachment;

#[poise::command(slash_command, prefix_command)]
pub async fn qr(ctx: Context<'_>, #[description = "Text or URL to encode in QR"] input: String) -> Result<(), Error> {
    let code = QrCode::new(input.as_bytes())
        .map_err(|e| Error::Other(anyhow::anyhow!("Failed to generate QR code: {}", e)))?;

    let image = code.render::<Luma<u8>>().build();
    let mut buffer = Cursor::new(Vec::new());

    image
        .write_to(&mut buffer, ImageFormat::Png)
        .map_err(|e| Error::Other(anyhow::anyhow!("Failed to write image: {}", e)))?;
    let image_bytes = buffer.into_inner();

    let attachment = CreateAttachment::bytes(image_bytes, "qrcode.png")
        .description(input);

    ctx.send(
        poise::CreateReply::default()
            .content("Here's your QR code:")
            .attachment(attachment),
    )
    .await?;

    Ok(())
}

