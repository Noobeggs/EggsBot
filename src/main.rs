#![warn(clippy::str_to_string)]

mod commands;

use anyhow::anyhow;

use poise::serenity_prelude as serenity;
use std::{collections::HashMap, env::var, sync::Mutex, time::Duration};

use shuttle_runtime;
use shuttle_secrets::SecretStore;
use shuttle_poise::ShuttlePoise;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command '{}': {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[shuttle_runtime::main]
pub async fn poise(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttlePoise<Data, Error> {
    env_logger::init();
    let discord_token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
            return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::mihoyo::genshin_codes(),
            commands::mihoyo::starrail_codes(),
            // commands::getvotes(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot"),
                poise::Prefix::Literal("hey bot,"),
            ],
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}...", ctx.command().qualified_name);
            })
        },
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!("Got an event in event handler: {:?}", event.name());
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .token(discord_token)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;
    Ok(framework.into())
}
