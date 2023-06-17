mod bancho;
mod get_memory;
mod structs;
use get_memory::get_osu_info;
use regex::Regex;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::transport::tcp::TCPTransport;
use twitch_irc::transport::tcp::TLS;
use twitch_irc::TwitchIRCClient;

use crate::structs::Config;

use self::structs::BM;
pub async fn catch_message(
    message: ServerMessage,
    client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
    config: Config,
) {
    let reg = Regex::new(r"(?:http:\/\/|https:\/\/)?(osu\.ppy\.sh\/)(beatmapsets|b)\/([0-9]*)#?(osu|taiko|catch|mania)?\/?([0-9]*)?\/?\+?([\S]*)?").unwrap();
    match message {
        ServerMessage::Privmsg(msg) => match msg.message_text.as_str() {
            "!skin" | "!cs" | "!скин" => {
                let osu_info = get_osu_info().await;
                if osu_info.error {
                    return client
                        .say(
                            config.twitch_channel_name.clone(),
                            "Bot hook is broken. I can't see song. BibleThump".to_owned(),
                        )
                        .await
                        .unwrap();
                }
                let osu_info = osu_info.osu_info.unwrap();
                return client
                    .say(
                        config.twitch_channel_name.clone(),
                        format!(
                            "Current skin: {} GlitchCat",
                            osu_info.settings.unwrap().folders.skin
                        ),
                    )
                    .await
                    .unwrap();
            }
            "!np" | "!map" | "!nowplay" | "!нп" | "!сонг" => {
                let osu_info = get_osu_info().await;
                if osu_info.error {
                    return client
                        .say(
                            config.twitch_channel_name.clone(),
                            "Bot hook is broken. I can't see skin folder. BibleThump".to_owned(),
                        )
                        .await
                        .unwrap();
                }
                let osu_info = osu_info.osu_info.unwrap();
                return client
                    .say(
                        config.twitch_channel_name.clone(),
                        format!(
                            "{} - {} [{}] +{} {} ☆ by {} | Download: osu.ppy.sh/b/{}#osu/{}",
                            osu_info.menu.to_owned().unwrap().bm.metadata.artist,
                            osu_info.menu.to_owned().unwrap().bm.metadata.title,
                            osu_info.menu.to_owned().unwrap().bm.metadata.difficulty,
                            osu_info.menu.to_owned().unwrap().mods.mod_str,
                            osu_info.menu.to_owned().unwrap().bm.stats.full_star_rate,
                            osu_info.menu.to_owned().unwrap().bm.metadata.mapper,
                            osu_info.menu.to_owned().unwrap().bm.id,
                            osu_info.menu.to_owned().unwrap().bm.set,
                        ),
                    )
                    .await
                    .unwrap();
            }
            "!pp" | "!nppp" => {
                let osu_info = get_osu_info().await;
                if osu_info.error {
                    return client
                        .say(
                            config.twitch_channel_name.clone(),
                            "Bot hook is broken. I can't see song. BibleThump".to_owned(),
                        )
                        .await
                        .unwrap();
                }
                let osu_info = osu_info.osu_info.unwrap();
                return client
                    .say(
                        config.twitch_channel_name.clone(),
                        format!(
                            "100%: {} | 99%: {} | 98%: {} | 97%: {} | 96%: {} | 95%: {}",
                            osu_info.menu.to_owned().unwrap().pp.full,
                            osu_info.menu.to_owned().unwrap().pp.dd,
                            osu_info.menu.to_owned().unwrap().pp.dv,
                            osu_info.menu.to_owned().unwrap().pp.ds,
                            osu_info.menu.to_owned().unwrap().pp.dsh,
                            osu_info.menu.to_owned().unwrap().pp.dp,
                        ),
                    )
                    .await
                    .unwrap();
            }
            st => {
                if reg.is_match(st) {
                    let caps = reg.captures(st).unwrap();
                    let mut bm: BM = BM { map: None };
                    match caps.get(5) {
                        Some(v) => {
                            match reqwest::get(format!(
                                "https://osu.ppy.sh/api/get_beatmaps?k={}&b={}",
                                config.osu_api_key.clone(),
                                v.as_str()
                            ))
                            .await
                            {
                                Ok(resp) => match resp.json::<Vec<structs::Beatmap>>().await {
                                    Ok(r) => {
                                        bm = BM { map: Some(r) };
                                    }
                                    Err(e) => {
                                        println!("{:?}", e);
                                    }
                                },
                                Err(e) => {
                                    println!("{:?}", e);
                                }
                            }
                        }
                        _ => {}
                    }
                    let beatmap;
                    match bm.map {
                        Some(r) => {
                            beatmap = r[0].to_owned();
                        }
                        _ => {
                            return;
                        }
                    }
                    client
                        .say(
                            config.twitch_channel_name.clone(),
                            format!(
                                "Request added: {} - {} {:.2}☆ [{}] by {}",
                                beatmap.artist,
                                beatmap.title,
                                beatmap.difficultyrating.parse::<f32>().unwrap(),
                                beatmap.version,
                                beatmap.creator
                            ),
                        )
                        .await
                        .unwrap();
                    let total_length = beatmap.total_length.parse::<f32>().unwrap();
                    bancho::bancho(format!(
                        "[{}] >> [http://osu.ppy.sh/b/{} {} - {} [{}]] {:.2}☆ {:.1} BPM {:.1}AR {:.1}OD {:0>2}:{:0>2}♫",
                        msg.sender.name,
                        beatmap.beatmap_id,
                        beatmap.artist,
                        beatmap.title,
                        beatmap.version,
                        beatmap.difficultyrating.parse::<f32>().unwrap(),
                        beatmap.bpm.parse::<f32>().unwrap(),
                        beatmap.diff_approach.parse::<f32>().unwrap(),
                        beatmap.diff_overall.parse::<f32>().unwrap(),
                        (total_length / 60.0).floor(),
                        (total_length - (total_length / 60.0).floor() * 60.0)
                    ), config.clone())
                    .await
                    .unwrap();
                }
            }
        },
        _ => {}
    }
}
