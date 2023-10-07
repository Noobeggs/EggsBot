#![warn(clippy::str_to_string)]

mod commands;

use anyhow::Context as _;

use poise::serenity_prelude as serenity;
use poise::event::Event;
use std::collections::BTreeMap;
use std::time::Duration;

use shuttle_runtime;
use shuttle_secrets::SecretStore;
use shuttle_poise::ShuttlePoise;

type Error = Box<dyn std::error::Error + Send + Sync>;
// type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    secrets: BTreeMap<String, String>,
    http_client: reqwest::Client,
}


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
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let llama_url = secret_store
        .get("LLAMA_URL")
        .context("'LLAMA_URL' was not found")?;

    let mut global_map: std::collections::btree_map::BTreeMap<String, String> = std::collections::BTreeMap::new();
    global_map.insert("llama_url".to_string(), llama_url);

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::mihoyo::genshin_codes(),
            commands::mihoyo::starrail_codes(),
            commands::chat::uwuify(),
            commands::chat::uwuify_context_menu(),
            commands::llama::llama(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            // additional_prefixes: vec![],
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                ctx.set_invocation_data("test").await;

                Ok(true)
            })
        }),
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}...", ctx.command().qualified_name);
            })
        },
        event_handler: |ctx, event, _framework, _data| {
            Box::pin(async move {
                println!("Got an event in event handler: {:?}", event.name());
                match event {
                    Event::Message { new_message }=> {
                        if !new_message.author.bot {
                            let _ = commands::chat::scan_message(ctx.clone(), new_message.clone()).await;
                        }
                    },
                    _ => ()
                }
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
                Ok(Data {
                    secrets: global_map,
                    http_client: reqwest::Client::new(),
                })
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
