use std::collections::HashMap;

use crate::Error;
use crate::Data;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct LlamaResponse {
    response: String
}

#[poise::command(prefix_command)]
pub async fn llama(
    ctx: poise::Context<'_, Data, Error>,
    message: String
) -> Result<(), Error> {
    let mut map = HashMap::new();
    map.insert("chat", message);

    let res: LlamaResponse = ctx.data().http_client.post(ctx.data().secrets.get("llama_url").unwrap())
        .json(&map)
        .send()
        .await?
        .json()
        .await?;
    
    ctx.send(|b| {
        b.content(res.response)
            .reply(true)
    })
    .await?;
    Ok(())
}
