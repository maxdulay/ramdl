use core::str;

use crate::api::*;
use base64::Engine;
use m3u8_rs::MasterPlaylist;

use crate::error::Result;

#[derive(Debug)]
pub struct StreamInfo {
    pub stream_url: String,
    pub pssh: String,
    pub codec: String,
}

impl StreamInfo {
    pub fn new(m3u8: Vec<u8>, base_uri: &str) -> Result<Self> {
        let (_, m3u8_data) = m3u8_rs::parse_master_playlist(&m3u8)
            .map_err(|e| crate::error::Error::Decrypt(e.to_string()))?;
        let drm_infos = Self::get_drm_info(&m3u8_data)?;
        let assert_info = Self::get_assert_info(&m3u8_data)?;
        let base_uri = match base_uri.rfind('/') {
            Some(index) => &base_uri[..index + 1],
            None => base_uri,
        };
        let stream_url = format!("{}{}", base_uri, m3u8_data.variants[0].uri.clone());
        let variant_id = m3u8_data.variants[0]
            .other_attributes
            .as_ref()
            .and_then(|attrs| attrs.get("STABLE-VARIANT-ID").cloned())
            .expect("STABLE-VARIANT-ID not found");
        let drm_ids = &assert_info[variant_id.as_str()]["AUDIO-SESSION-KEY-IDS"];
        let pssh = StreamInfo::get_pssh(&drm_infos, drm_ids).unwrap().clone();
        let codec = m3u8_data.variants[0].codecs.as_ref().unwrap().clone();
        Ok(Self {
            stream_url,
            pssh,
            codec,
        })
    }

    // pub async fn new_with_song(song: Song, client: &reqwest::Client) -> Result<Self> {
    //     let m3u8_url = song.attributes.extended_asset_urls.enhanced_hls;
    //     let m3u8 = client.get(&m3u8_url).send().await?.bytes().await?.to_vec();
    //     let stream_info = Self::new(m3u8, &m3u8_url);
    //     stream_info
    // }

    pub async fn new_with_webplayback(webplayback: &webplayback::WebPlayBack) -> Result<Self> {
        let webplayback = webplayback
            .song_list
            .first()
            .unwrap()
            .assets
            .iter()
            .find(|t| t.flavor == "28:ctrp256")
            .unwrap();

        let m3u8 = reqwest::get(&webplayback.url)
            .await?
            .bytes()
            .await?
            .to_vec();

        let media_playlist = m3u8_rs::parse_media_playlist(&m3u8)
            .map_err(|e| crate::error::Error::Decrypt(e.to_string()))?
            .1;

        if let Some(pssh) = media_playlist
            .segments
            .first()
            .and_then(|segment| segment.key.as_ref())
            .and_then(|key| key.uri.as_ref())
        {
            Ok(Self {
                stream_url: webplayback.url.clone(),
                pssh: pssh.clone(),
                codec: String::new(),
            })
        } else {
            Err(crate::error::Error::Other("Source not exists".to_string()))
        }
    }

    fn get_drm_info(m3u8_data: &MasterPlaylist) -> Result<serde_json::Value> {
        let drm_info = m3u8_data
            .session_data
            .iter()
            .find(|data| data.data_id == "com.apple.hls.AudioSessionKeyInfo")
            .ok_or_else(|| crate::error::Error::Decrypt("DATA-ID not found".to_string()))?;
        if let m3u8_rs::SessionDataField::Value(value) = &drm_info.field {
            let drm_info = base64::engine::general_purpose::STANDARD.decode(value)?;
            Ok(serde_json::from_slice(&drm_info)?)
        } else {
            Err(crate::error::Error::Decrypt(
                "DATA-ID not found".to_string(),
            ))
        }
    }

    fn get_assert_info(m3u8_data: &MasterPlaylist) -> Result<serde_json::Value> {
        let assert_info = m3u8_data
            .session_data
            .iter()
            .find(|data| data.data_id == "com.apple.hls.audioAssetMetadata")
            .ok_or_else(|| crate::error::Error::Decrypt("DATA-ID not found".to_string()))?;
        if let m3u8_rs::SessionDataField::Value(value) = &assert_info.field {
            let assert_info = base64::engine::general_purpose::STANDARD.decode(value)?;
            Ok(serde_json::from_slice(&assert_info)?)
        } else {
            Err(crate::error::Error::Decrypt(
                "DATA-ID not found".to_string(),
            ))
        }
    }

    fn get_pssh(drm_infos: &serde_json::Value, drm_ids: &serde_json::Value) -> Option<String> {
        for drm_id in drm_ids.as_array()? {
            if let Some(drm_info) = drm_infos.get(drm_id.as_str()?) {
                if let Some(uri) = drm_info.get("urn:uuid:edef8ba9-79d6-4ace-a3c8-27dcd51d21ed") {
                    if drm_id != "1" {
                        return uri
                            .get("URI")
                            .and_then(|u| u.as_str().map(|s| s.to_string()));
                    }
                }
            }
        }
        None
    }
}
