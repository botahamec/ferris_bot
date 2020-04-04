mod events;

use events::Handler;

use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }, Args
};

use std::fs::File;
use std::io::prelude::*;

#[group]
#[commands(ping, say)]
struct General;

fn main() {
	let mut file = File::open("token").unwrap();
	let mut token = String::new();
	file.read_to_string(&mut token)
		.expect("Token file not found");

	let mut client =
		Client::new(&token, Handler).expect("Error creating client");

	client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("?"))
        .group(&GENERAL_GROUP));

	if let Err(msg) = client.start() {
		println!("Error: {:?}", msg);
	}
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
	msg.reply(&ctx, "Pong?")?;

	Ok(())
}

#[command]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	msg.channel_id.say(&ctx.http, args.rest())?;

	Ok(())
}