extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;

const TOKEN : &str = "Njg4NDU4OTc0Njk3NDg4NTI5.Xm0ncg.VSdgaMN8B2MZvpf9TclDwayA9Os";

struct Handler;

impl EventHandler for Handler {

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

fn main() {

    let mut client = Client::new(&TOKEN, Handler)
                        .expect("Error creating client");

    if let Err(msg) = client.start() {
        println!("Error: {:?}", msg);
    }
}
