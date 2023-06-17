use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Beatmap {
    pub approved: String,
    pub beatmap_id: String,
    pub artist: String,
    pub title: String,
    pub version: String,
    pub difficultyrating: String,
    pub bpm: String,
    pub diff_approach: String,
    pub diff_overall: String,
    pub total_length: String,
    pub creator: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BM {
    pub map: Option<Vec<Beatmap>>,
}
