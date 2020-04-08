mod commands;
mod events;

use commands::GENERAL_GROUP;
use commands::EMOJI_GROUP;
use commands::ADMIN_GROUP;
use events::Handler;

use serenity::client::Client;
use serenity::framework::standard::StandardFramework;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let mut file = File::open("token").unwrap();
	let mut token = String::new();
	file.read_to_string(&mut token)
		.expect("Token file not found");

	let mut client =
		Client::new(&token, Handler).expect("Error creating client");

	let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            owners
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

	client.with_framework(
		StandardFramework::new()
			.configure(|c| c.prefix("?").owners(owners))
			.group(&GENERAL_GROUP)
			.group(&EMOJI_GROUP)
			.group(&ADMIN_GROUP),
	);

	if let Err(msg) = client.start() {
		println!("Error: {:?}", msg);
	}
}
