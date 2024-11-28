pub mod api;
pub mod decrypter;
pub mod error;
pub mod stream_info;

use crate::api::*;
use crate::error::Error;
use crate::error::Result;
use base64::Engine;
use fancy_regex::Regex;
use serde_json::json;

use lyrics::Lyrics;
use search;
use songs::Song;

pub const APPLE_MUSIC_HOMEPAGE_URL: &str = "https://beta.music.apple.com";
pub const AMP_API_URL: &str = "https://amp-api.music.apple.com";
pub const WEBPLAYBACK_API_URL: &str =
    "https://play.itunes.apple.com/WebObjects/MZPlay.woa/wa/webPlayback";
pub const LICENSE_API_URL: &str =
    "https://play.itunes.apple.com/WebObjects/MZPlay.woa/wa/acquireWebPlaybackLicense";

pub struct AppleMusicDownloader {
    media_user_token: String,
    store_front: String,
    language: String,
    headers: reqwest::header::HeaderMap,
    client: reqwest::Client,
}

impl Default for AppleMusicDownloader {
    fn default() -> Self {
        AppleMusicDownloader {
            media_user_token: "".to_string(),
            store_front: "us".to_string(),
            language: "en-US".to_string(),
            headers: reqwest::header::HeaderMap::new(),
            client: reqwest::Client::new(),
        }
    }
}

impl AppleMusicDownloader {
    pub fn new(media_user_token: &str, store_front: &str, language: &str) -> Self {
        Self {
            media_user_token: media_user_token.to_string(),
            store_front: store_front.to_string(),
            language: language.to_string(),
            ..Default::default()
        }
    }

    pub async fn new_with_media_user_token(media_user_token: &str) -> Result<Self> {
        let mut apple_music = AppleMusicDownloader {
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

    pub async fn get_songs(&self, song_id: &str) -> Result<Song> {
        let store_front = self.store_front.clone();
        let res = self
            .client
            .get(format!(
                "{AMP_API_URL}/v1/catalog/{store_front}/songs/{song_id}?include=albums&extend=extendedAssetUrls",
            ))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let song: Song = serde_json::from_value(res["data"][0].clone())?;
        Ok(song)
    }

    pub async fn get_lyrics(&self, song_id: &str) -> Result<Vec<Option<Lyrics>>> {
        let store_front = self.store_front.clone();
        let res = self
            .client
            .get(format!(
                "{AMP_API_URL}/v1/catalog/{store_front}/songs/{song_id}?include=lyrics,syllable-lyrics&extend=extendedAssetUrls",
            ))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let lyrics: Option<Lyrics> =
            serde_json::from_value(res["data"][0]["relationships"]["lyrics"].clone()).ok();
        let syllable_lyrics: Option<Lyrics> =
            serde_json::from_value(res["data"][0]["relationships"]["syllable-lyrics"].clone()).ok();
        Ok(vec![lyrics, syllable_lyrics])
    }

    pub async fn search(&self, query: &str) -> Result<search::SearchResults> {
        let store_front = self.store_front.clone();
        let res = self
            .client
            .get(format!(
                "{AMP_API_URL}/v1/catalog/{store_front}/search?term={query}&types=songs,albums,artists,playlists&limit=25&offset=0",
            ))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let songs: Vec<search::Song> =
            serde_json::from_value(res["results"]["songs"]["data"].clone())?;
        let albums: Vec<search::Album> =
            serde_json::from_value(res["results"]["albums"]["data"].clone())?;
        let artists: Vec<search::Artist> =
            serde_json::from_value(res["results"]["artists"]["data"].clone())?;
        let playlists: Vec<search::Playlist> =
            serde_json::from_value(res["results"]["playlists"]["data"].clone())?;
        Ok(search::SearchResults {
            songs,
            albums,
            artists,
            playlists,
        })
    }

    pub async fn get_widevine_license(
        &self,
        track_id: &str,
        track_uri: &str,
        challenge: Vec<u8>,
    ) -> Result<Vec<u8>> {
        let challenge_str = base64::engine::general_purpose::STANDARD.encode(&challenge);
        let response = self
            .client
            .post(LICENSE_API_URL)
            .json(&serde_json::json!({
                "challenge": challenge_str,
                "key-system": "com.widevine.alpha",
                "uri": track_uri,
                "adamId": track_id,
                "isLibrary": false,
                "user-initiated": true,
            }))
            .send()
            .await?;

        if response.status().is_success() {
            let response_dict: serde_json::Value = response.json().await?;
            if let Some(widevine_license) = response_dict.get("license") {
                let license = base64::engine::general_purpose::STANDARD
                    .decode(widevine_license.as_str().unwrap())?;
                return Ok(license);
            }
        }

        Err(Error::Init("Failed to get Widevine license".to_string()))
    }

    pub async fn get_webplayback(&self, track_id: &str) -> Result<webplayback::WebPlayBack> {
        let response = self
            .client
            .post(WEBPLAYBACK_API_URL)
            .body(
                json!({
                    "salableAdamId": track_id,
                    "language": self.language,
                })
                .to_string(),
            )
            .send()
            .await
            .unwrap()
            .json::<webplayback::WebPlayBack>()
            .await
            .unwrap();
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use stream_info::StreamInfo;

    use crate::decrypter;

    use super::*;

    #[tokio::test]
    async fn test_apple_music() {
        let args: Vec<String> = std::env::args().collect();
        let media_user_token = args
            .get(2)
            .expect("MEDIA_USER_TOKEN not provided")
            .to_string();
        let apple_music_downloader =
            AppleMusicDownloader::new_with_media_user_token(&media_user_token)
                .await
                .unwrap();
        let device_private_key = include_bytes!("../device/private_key.pem").to_vec();
        let device_client_id_blob = include_bytes!("../device/client_id.bin").to_vec();
        let mut decrypter = decrypter::Decrypter::new(device_private_key, device_client_id_blob);
        let webplayback = apple_music_downloader
            .get_webplayback("1753050648")
            .await
            .unwrap();
        let stream_info = StreamInfo::new_with_webplayback(&webplayback)
            .await
            .unwrap();
        let decrypt_key = decrypter
            .get_decrypt_key(&stream_info, &apple_music_downloader)
            .await
            .unwrap();
        assert_eq!(decrypt_key.len(), 32);
    }
}
