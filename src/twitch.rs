use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
mod message_catcher;
use message_catcher::catch_message;

use crate::structs::Config;
pub async fn twitch(config_d: Config) {
    let login_name = config_d.twitch_bot_username.to_owned();
    let token = config_d.twitch_bot_token.replace("oauth:", "");
    let config = ClientConfig::new_simple(StaticLoginCredentials::new(login_name, Some(token)));
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
    client
        .to_owned()
        .join(config_d.twitch_channel_name.clone())
        .unwrap();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            catch_message(message, client.to_owned(), config_d.clone()).await;
        }
    });
    join_handle.await.unwrap();
}
