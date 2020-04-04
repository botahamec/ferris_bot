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

<<<<<<< HEAD
#[group]
#[commands(ping, say)]
struct General;
=======
struct Handler;

impl EventHandler for Handler {
	fn reaction_add(&self, ctx: Context, reaction: Reaction) {
		if let Err(why) = reaction.channel_id.say(
			&ctx.http,
			format!(
				"{} left a {} reaction",
				reaction.user(&ctx).unwrap().name,
				match reaction.emoji {
					ReactionType::Custom {
						animated: _animated,
						id: _id,
						name,
					} => name.unwrap(),
					ReactionType::Unicode(uni) => uni,
					ReactionType::__Nonexhaustive => String::new(),
				}
			),
		) {
			println!("Error reacting to a reaction: {:?}", why);
		}
	}

	fn message(&self, ctx: Context, msg: Message) {
		if msg.content == "?ping" {
			if let Err(why) = msg.channel_id.say(&ctx.http, "Pong?") {
				println!("Error giving message: {:?}", why);
			}
		}
	}

	fn ready(&self, _: Context, ready: Ready) {
		println!("{} is ready", ready.user.name);
	}
}
>>>>>>> f60ddd8fd15002f83e02c8f28aeb4b9fb078ba18

fn main() {
	let mut file = File::open("token").unwrap();
	let mut token = String::new();
	file.read_to_string(&mut token)
		.expect("Token file not found");
<<<<<<< HEAD

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
=======

	let mut client =
		Client::new(&token, Handler).expect("Error creating client");

	if let Err(msg) = client.start() {
		println!("Error: {:?}", msg);
	}
>>>>>>> f60ddd8fd15002f83e02c8f28aeb4b9fb078ba18
}

#[command]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	msg.channel_id.say(&ctx.http, args.rest())?;

	Ok(())
}