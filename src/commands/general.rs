use serenity::{
	framework::standard::{
		macros::{command, group},
		Args, CommandResult,
	},
	model::channel::Message,
	prelude::Context,
};


#[group]
#[commands(ping, say)]
struct General;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
	msg.reply(&ctx, "Pong?")?;
	Ok(())
}

#[command]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	msg.reply(&ctx, args.rest())?;
	Ok(())
}
