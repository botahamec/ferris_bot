use serenity::{
	framework::standard::{
		macros::{command, group},
		CommandResult,
	},
	model::channel::Message,
	prelude::Context,
};

#[group]
#[commands(bird, man, onion)]
#[description = "Sends out an emoji in the chat"]
#[default_command(bird)]
#[prefixes("em", "emoji")]
struct Emoji;

#[command]
fn bird(ctx: &mut Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, "🐦")?;
	Ok(())
}

#[command]
fn man(ctx: &mut Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, "🕴️")?;
	Ok(())
}

#[command]
fn onion(ctx: &mut Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, "🧅")?;
	Ok(())
}