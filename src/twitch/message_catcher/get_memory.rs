mod structs;
use structs::*;
pub async fn get_osu_info() -> GOsuMemoryResponse {
    let err = GOsuMemoryResponse {
        error: true,
        osu_info: None,
    };
    match reqwest::get("http://localhost:24050/json").await {
        Ok(resp) => match resp.json::<OsuInfo>().await {
            Ok(r) => GOsuMemoryResponse {
                error: false,
                osu_info: Some(r),
            },
            Err(_) => err,
        },
        Err(_) => err,
    }
}
