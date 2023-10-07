#![warn(clippy::str_to_string)]

mod commands;
pub mod singletons {
    pub static HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

use poise::serenity_prelude as serenity;
use poise::event::Event;
use std::collections::BTreeMap;
use std::time::Duration;

use std::{collections::HashMap, env::var, sync::Mutex, time::Duration};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    secrets: BTreeMap<String, String>,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenvy::dotenv()?;
    env_logger::init();

    let llama_url = var("LLAMA_URL").expect("Missing `LLAMA_URL` env var, see README for more information.");

    let mut global_map: std::collections::btree_map::BTreeMap<String, String> = std::collections::BTreeMap::new();
    global_map.insert("llama_url".to_string(), llama_url);

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::mihoyo::genshin_codes(),
            commands::mihoyo::starrail_codes(),
            commands::chat::uwuify(),
            commands::chat::uwuify_context_menu(),
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

    poise::Framework::builder()
        .token(
            var("DISCORD_TOKEN")
                .expect("Missing `DISCORD_TOKEN` env var, see README for more information."),
        )
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    secrets: global_map,
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await
        .unwrap();
    Ok(())
}
