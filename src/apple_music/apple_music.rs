use std::path::PathBuf;

use crate::error::Error;
use crate::error::Result;
use fancy_regex::Regex;

use super::song::Song;

pub const APPLE_MUSIC_HOMEPAGE_URL: &str = "https://beta.music.apple.com";
pub const AMP_API_URL: &str = "https://amp-api.music.apple.com";
pub const WEBPLAYBACK_API_URL: &str =
    "https://play.itunes.apple.com/WebObjects/MZPlay.woa/wa/webPlayback";
pub const LICENSE_API_URL: &str =
    "https://play.itunes.apple.com/WebObjects/MZPlay.woa/wa/acquireWebPlaybackLicense";

pub struct AppleMusic {
    media_user_token: String,
    store_front: String,
    language: String,
    headers: reqwest::header::HeaderMap,
    client: reqwest::Client,
}

impl Default for AppleMusic {
    fn default() -> Self {
        AppleMusic {
            media_user_token: "".to_string(),
            store_front: "us".to_string(),
            language: "en-US".to_string(),
            headers: reqwest::header::HeaderMap::new(),
            client: reqwest::Client::new(),
        }
    }
}

impl AppleMusic {
    pub async fn new_with_media_user_token(media_user_token: &str) -> Result<Self> {
        let mut apple_music = AppleMusic {
            media_user_token: media_user_token.to_string(),
            ..Default::default()
        };
        apple_music.init().await?;
        Ok(apple_music)
    }

    async fn init(&mut self) -> Result<()> {
        self.init_session().await?;
        self.init_headers().await?;
        self.init_storefront_language().await?;
        Ok(())
    }

    async fn init_session(&mut self) -> Result<()> {
        let home_page = self
            .client
            .get(APPLE_MUSIC_HOMEPAGE_URL)
            .send()
            .await?
            .text()
            .await?;
        let js_re = Regex::new(r#"(?<=index)(.*?)(?=\.js")"#).unwrap();
        let js_file = js_re
            .find(&home_page)?
            .map(|value| value.as_str())
            .ok_or(Error::Init("Parsing home page error".to_string()))?;
        let js_res = reqwest::get(format!(
            "{APPLE_MUSIC_HOMEPAGE_URL}/assets/index{js_file}.js"
        ))
        .await
        .unwrap();
        let js_res_text = js_res.text().await.unwrap();

        let token_re = Regex::new(r#"(?=eyJh)(.*?)(?=")"#).unwrap();
        let token = token_re
            .find(&js_res_text)?
            .map(|value| value.as_str())
            .ok_or(Error::Init("Parsing home page error".to_string()))?;
        self.headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse().unwrap(),
        );
        self.client = reqwest::Client::builder()
            .default_headers(self.headers.clone())
            .build()?;
        Ok(())
    }

    async fn init_headers(&mut self) -> Result<()> {
        self.headers.insert(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:95.0) Gecko/20100101 Firefox/95.0"
                .parse()
                .unwrap(),
        );
        self.headers
            .insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
        self.headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        self.headers.insert(
            reqwest::header::HeaderName::from_static("media-user-token"),
            self.media_user_token.parse().unwrap(),
        );
        self.headers.insert(
            reqwest::header::ORIGIN,
            APPLE_MUSIC_HOMEPAGE_URL.parse().unwrap(),
        );
        self.client = reqwest::Client::builder()
            .default_headers(self.headers.clone())
            .build()?;
        Ok(())
    }

    async fn init_storefront_language(&mut self) -> Result<()> {
        let res = self
            .client
            .get(format!("{AMP_API_URL}/v1/me/storefront"))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        self.store_front = res["data"][0]["id"].as_str().unwrap().to_string();
        self.language = res["data"][0]["attributes"]["defaultLanguageTag"]
            .as_str()
            .unwrap()
            .to_string();
        Ok(())
    }

    pub async fn get_song(&self, song_id: &str) -> Result<Song> {
        let store_front = self.store_front.clone();
        let res = self
            .client
            .get(format!(
                "{AMP_API_URL}/v1/catalog/{store_front}/songs/{song_id}?include=lyrics,albums&extend=extendedAssetUrls",
            ))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let song: Song = serde_json::from_value(res["data"][0].clone())?;
        Ok(song)
    }
}

pub trait AppleMusicItem {
    fn download(dir: PathBuf) -> impl std::future::Future<Output = Result<()>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_apple_music() {
        let args: Vec<String> = std::env::args().collect();
        let media_user_token = args
            .get(2)
            .expect("MEDIA_USER_TOKEN not provided")
            .to_string();
        let apple_music = AppleMusic::new_with_media_user_token(&media_user_token)
            .await
            .unwrap();
        assert_eq!(apple_music.store_front, "jp");
        assert_eq!(apple_music.language, "ja");
    }
}
