use crate::Error;
use crate::Data;
use poise::say_reply;
use poise::serenity_prelude as serenit;
use uwuifier::uwuify_str_sse;

/// Check for the word soon in each message
fn soon(message: &String) -> bool {
    if message.len() > 250 {
        return false;
    }
    let iter = message.split_whitespace();
    for word in iter {
        if word == "soon" {
            return true;
        }
    }
    return false;
}

/// Reply with "keat" if "soon" was detected
pub async fn scan_message(
    ctx: serenit::Context,
    new_message: serenit::Message
) -> Result<(), Error> {
    let message = &new_message.content;
    if soon(&message) {
        let reply = "> soon\nkeat";
        let _ = new_message.reply_ping(ctx, reply).await;
    }
    Ok(())
}

/// Uwuify text from context menu
#[poise::command(context_menu_command = "Uwuify")]
pub async fn uwuify_context_menu(
    ctx: poise::Context<'_, Data, Error>,
    new_message: serenit::Message,
) -> Result<(), Error> {
    let message = new_message.content;
    let _ = say_reply(ctx, uwuify_str_sse(&message)).await;
    Ok(())
}

/// Uwuify prefix command
/// Uwuifies
#[poise::command(prefix_command, aliases("uwu"))]
pub async fn uwuify(
    ctx: poise::Context<'_, Data, Error>,
) -> Result<(), Error> {
    // If there is a referenced message, uwuify it!
    if let poise::Context::Prefix(prefix_context) = ctx {
        if let Some(boxed_message) = &prefix_context.msg.referenced_message {
            let quote = &boxed_message.content;
            let _ = say_reply(ctx, uwuify_str_sse(&quote)).await;
        } else {
            let _ = say_reply(ctx, "Please reply to a message to uwuify (´ ꒳ ` ✿)").await;
        }
    }
    Ok(())
}
