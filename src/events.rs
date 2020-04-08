use serenity::{
	model::{
		channel::{Message, Reaction, ReactionType},
		gateway::Ready,
	},
	prelude::{Context, EventHandler},
};

pub struct Handler;

impl EventHandler for Handler {
	/* says pong on "!ping"
	fn message(&self, ctx: Context, msg: Message) {
		if msg.content == "?ping" {
			if let Err(why) = msg.channel_id.say(&ctx.http, "Pong?") {
				println!("Error giving message: {:?}", why);
			}
		}
	}*/

	// says a message whenever a reaction is made
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

	// prints a message to the console when ready
	fn ready(&self, _: Context, ready: Ready) {
		println!("{} is ready", ready.user.name);
	}
}
