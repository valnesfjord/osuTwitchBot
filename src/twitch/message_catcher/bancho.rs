use futures::prelude::*;
use irc::client::prelude::*;
use std::error::Error;

pub async fn bancho(msg: String, conf: crate::structs::Config) -> Result<(), Box<dyn Error>> {
    let config = Config {
        use_tls: Some(false),
        password: Some(conf.osu_irc_password.clone()),
        server: Some("irc.ppy.sh".to_owned()),
        port: Some(6667),
        nickname: Some(conf.osu_username.clone()),
        username: Some(conf.osu_username.clone()),
        ..Config::default()
    };
    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let mut i = 0;
    while let Some(message) = stream.next().await.transpose()? {
        if let Command::JOIN(_, _, _) = message.command {
        } else if let Command::PART(_, _) = message.command {
        } else if let Command::QUIT(_) = message.command {
        } else {
            if i == 21 {
                client
                    .send_privmsg(client.current_nickname(), msg.as_str())
                    .unwrap();
            } else if i == 22 {
                client.send_quit("quit").unwrap();
                return Ok(());
            }
            i = i + 1;
        }
    }

    Ok(())
}
