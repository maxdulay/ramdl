#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Lyrics {
    pub href: String,
    pub data: Vec<LyricData>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LyricData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub attributes: LyricAttributes,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LyricAttributes {
    pub ttml: String,
    pub play_params: PlayParams,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayParams {
    pub id: String,
    pub kind: String,
    pub catalog_id: String,
    pub display_type: i32,
}

#[cfg(test)]
mod tests {
    use crate::AppleMusicDownloader;

    #[tokio::test]
    async fn test_lyrics() {
        let args: Vec<String> = std::env::args().collect();
        let media_user_token = args
            .get(2)
            .expect("MEDIA_USER_TOKEN not provided")
            .to_string();
        let apple_music = AppleMusicDownloader::new_with_media_user_token(&media_user_token)
            .await
            .unwrap();
        let lyrics = apple_music.get_lyrics("1428083880").await.unwrap();

        println!("{:#?}", lyrics);
    }
}
