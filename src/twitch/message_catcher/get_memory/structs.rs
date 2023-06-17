use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct OsuInfo {
    pub settings: Option<Settings>,
    pub menu: Option<Menu>,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Menu {
    pub state: i32,
    #[serde(rename = "gameMode")]
    pub game_mode: i32,
    pub bm: Beatmap,
    pub pp: PP,
    pub mods: Mods,
}
#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub folders: Folders,
}
#[derive(Deserialize, Serialize)]
pub struct Folders {
    pub skin: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct PP {
    #[serde(rename = "100")]
    pub full: i32,
    #[serde(rename = "99")]
    pub dd: i32,
    #[serde(rename = "98")]
    pub dv: i32,
    #[serde(rename = "97")]
    pub ds: i32,
    #[serde(rename = "96")]
    pub dsh: i32,
    #[serde(rename = "95")]
    pub dp: i32,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Mods {
    #[serde(rename = "str")]
    pub mod_str: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Metadata {
    pub artist: String,
    pub title: String,
    pub difficulty: String,
    pub mapper: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Stats {
    #[serde(rename = "fullSR")]
    pub full_star_rate: f32,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Beatmap {
    pub id: i32,
    pub set: i32,
    #[serde(rename = "rankedStatus")]
    pub approved: i32,
    pub metadata: Metadata,
    pub stats: Stats,
}

#[derive(Deserialize, Serialize)]
pub struct GOsuMemoryResponse {
    pub error: bool,
    pub osu_info: Option<OsuInfo>,
}
