use crate::Error;
use crate::Data;
use poise::serenity_prelude as serenit;
use serenity::utils::Colour;

#[poise::command(slash_command)]
pub async fn genshin_codes(
    ctx: poise::Context<'_, Data, Error>,
    #[description = "Codes separated by ',' (no spaces)"] codes: String,
    #[description = "Channel to send in"] channel: Option<serenit::Channel>,
    #[description = "Roles to ping"] role: Option<serenit::Role>
) -> Result<(), Error> {
    let (codes_vec, wrong_length) = extract_codes_from_string(codes);
    let n = wrong_length.len();
    if n != 0 {
        wrong_length_codes_found(ctx, wrong_length, n).await?;
    }
    let no_of_codes = codes_vec.len();
    if no_of_codes < 1 {
        no_valid_codes_found(ctx).await?;
        return Ok(());
    }
    // I use unwrap here because I checked that codes.len() > 0...
    // Let's see if this bites me back in the butt :D
    let codes_string = codes_vec.iter().map(|s| s.to_string()).reduce(|a, b| a + "\n" + &b).unwrap();
    let description = ["Codes:\n", &codes_string].join("\n");
    let send_channel = match channel {
        Some(chan) => chan.id(),
        None => ctx.channel_id(),
    };
    ctx.send(|b| {
        b.content("Codes sent!")
            .ephemeral(true)
    })
    .await?;
    send_channel.send_message(ctx.serenity_context(), |b| {
        let mut content = "".to_string();
        if let Some(r) = role {
            content = serenit::MessageBuilder::new()
                .push("📢 ")
                .mention(&r)
                .push(" • Redeemable Codes")
                .build();
        }
        b.content(content)
            .embed(|b| 
                b.description(description)
                    .title("Genshin Impact Codes")
                    .colour(Colour::BLITZ_BLUE))
                    .components(|b| {
                        b.create_action_row(|b| {
                            for i in 0..no_of_codes {
                                let code = codes_vec[i].clone();
                                b.add_button(genshin_redeem_url_button(code));
                            };
                            return b;
                        })
                    })
    })
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn starrail_codes(
    ctx: poise::Context<'_, Data, Error>,
    #[description = "Codes separated by ',' (no spaces)"] codes: String,
    #[description = "Channels to send in"] channel: Option<serenit::Channel>,
    #[description = "Roles to ping"] role: Option<serenit::Role>
) -> Result<(), Error> {
    let (codes_vec, wrong_length) = extract_codes_from_string(codes);
    let n = wrong_length.len();
    if n != 0 {
        wrong_length_codes_found(ctx, wrong_length, n).await?;
    }
    let no_of_codes = codes_vec.len();
    if no_of_codes < 1 {
        no_valid_codes_found(ctx).await?;
        return Ok(());
    }
    // I use unwrap here because I checked that codes.len() > 0...
    // Let's see if this bites me back in the butt :D
    let codes_string = codes_vec.iter().map(|s| s.to_string()).reduce(|a, b| a + "\n" + &b).unwrap();
    let description = ["Codes:\n", &codes_string].join("\n");
    let send_channel = match channel {
        Some(chan) => chan.id(),
        None => ctx.channel_id(),
    };
    ctx.send(|b| {
        b.content("Codes sent!")
            .ephemeral(true)
    })
    .await?;
    send_channel.send_message(ctx.serenity_context(), |b| {
        let mut content = "".to_string();
        if let Some(r) = role {
            content = serenit::MessageBuilder::new()
                .push("📢 ")
                .role(&r)
                .push(" • Redeemable Codes")
                .build();
        }
        b.content(content)
            .embed(|b| b.description(description)
                .title("Honkai: Star Rail Codes")
                .colour(Colour::BLITZ_BLUE))
                .components(|b| {
                    b.create_action_row(|b| {
                        for i in 0..no_of_codes {
                            let code = codes_vec[i].clone();
                            b.add_button(starrail_redeem_url_button(code));
                        };
                        return b;
                    })
                })
    })
    .await?;
    Ok(())
}

fn extract_codes_from_string(input: String) -> (Vec<String>, Vec<String>) {
    let codes: Vec<String> = input.split(',').map(str::to_string).collect();
    let (acceptable, wrong_length): (Vec<String>, Vec<String>) = codes.into_iter().partition(|x| x.len() == 12);
    return (acceptable, wrong_length);
}

async fn no_valid_codes_found(ctx: poise::Context<'_, Data, Error>) -> Result<(), Error> {
    ctx.send(|b| {
        b.content("No valid codes found!")
            .ephemeral(true)
    })
    .await?;
    Ok(())
}

async fn wrong_length_codes_found(ctx: poise::Context<'_, Data, Error>, not_codes: Vec<String>, length: usize) -> Result<(), Error> {
    let mut not_codes_string = "".to_string();
    for not_code in not_codes {
        not_codes_string += &not_code;
        not_codes_string.push('\n');
    }
    ctx.send(|b| {
        b.content(format!("{} codes that are of the wrong length found!\n{}", length, not_codes_string))
            .ephemeral(true)
    })
    .await?;
    Ok(())
}

// fn incorrect_length_codes(input: String) -> usize {
//     let codes: Vec<String> = input.split(',').map(str::to_string).collect();
//     codes.into_iter().filter(|x| x.len() != 12).count()
// }

fn genshin_redeem_url_button(code: String) -> serenit::CreateButton {
    let mut b = serenit::CreateButton::default();
    b.url(format!("https://genshin.hoyoverse.com/en/gift?code={}", &code));
    b.label(code);
    b.style(serenit::ButtonStyle::Link);
    b
}

fn starrail_redeem_url_button(code: String) -> serenit::CreateButton {
    let mut b = serenit::CreateButton::default();
    b.url(format!("https://hsr.hoyoverse.com/gift?code={}", &code));
    b.label(code);
    b.style(serenit::ButtonStyle::Link);
    b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_codes() {
        let codes0 = String::from("");
        let codes1 = String::from("NS92PG6DB52M,2SR3PY7CA52V");
        let codes3 = String::from("2SR3PY7CA52V,,QBQ2NH6DB4Z9");
        let codes4 = String::from("QBQ2NH6DB4Z9,ihatemylife,6A836GNUA52Z");

        let (res1, err1) = extract_codes_from_string(codes0);
        let (res2, err2) = extract_codes_from_string(codes1);
        let (res3, err3) = extract_codes_from_string(codes3);
        let (res4, err4) = extract_codes_from_string(codes4);

        assert_eq!(res1.len(), 0);
        assert_eq!(res2.len(), 2);
        assert_eq!(res3.len(), 2);
        assert_eq!(res4.len(), 2);
        assert_eq!(err1.len(), 1);
        assert_eq!(err2.len(), 0);
        assert_eq!(err3.len(), 1);
        assert_eq!(err4.len(), 1);
        
        assert_eq!(res2[0], "NS92PG6DB52M");       
        assert_eq!(res2[1], "2SR3PY7CA52V");
        assert_eq!(res3[0], "2SR3PY7CA52V");
        assert_eq!(res3[1], "QBQ2NH6DB4Z9");
        assert_eq!(res4[0], "QBQ2NH6DB4Z9");
        assert_eq!(res4[1], "6A836GNUA52Z");
        assert_eq!(err4[0], "ihatemylife");
    }
}
