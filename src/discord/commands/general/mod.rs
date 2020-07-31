use serenity::framework::standard::macros::group;

mod ping;
use ping::PING_COMMAND;

#[group]
#[commands(ping)]
struct General;
