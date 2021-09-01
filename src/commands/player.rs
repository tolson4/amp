use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;


#[command]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "PLAY").await?;

    Ok(())
}


#[command]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "QUEUE").await?;

    Ok(())
}

#[command]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "PAUSE").await?;

    Ok(())
}

#[command]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "RESUME").await?;

    Ok(())
}


#[command]
async fn next(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "NEXT").await?;

    Ok(())
}

#[command]
async fn previous(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "PREVIOUS").await?;

    Ok(())
}

#[command]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "STOP").await?;

    Ok(())
}
