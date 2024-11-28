use crate::{error::Result, AppleMusicDownloader};
use base64::Engine;
use pssh_box::{widevine::WidevinePsshData, PsshBox, PsshData, ToBytes};
use widevine::{self, Cdm, LicenseType, Pssh};

pub async fn get_decrypt_key(
    cdm: &Cdm,
    pssh: &str,
    id: &str,
    apple_music_downloader: &AppleMusicDownloader,
) -> Result<String> {
    let session = cdm.open();
    let key_id = base64::engine::general_purpose::STANDARD
        .decode(pssh.split(',').collect::<Vec<&str>>().last().unwrap())?;
    let widevine_pssh_data = WidevinePsshData {
        key_id: vec![key_id],
        algorithm: Some(1),
        ..Default::default()
    };
    let pssh_obj = PsshData::Widevine(widevine_pssh_data);
    let mut pssh_box = PsshBox::new_widevine();
    // let key_id: DRMKeyId = key_id.try_into().unwrap();
    // pssh_box.add_key_id(key_id);
    pssh_box.pssh_data = pssh_obj;
    pssh_box.version = 0;
    let pssh_obj = Pssh::from_bytes(&pssh_box.to_bytes()).unwrap();
    let cdm_license_request = session
        .get_license_request(pssh_obj, LicenseType::STREAMING)
        .unwrap();

    let license_message = apple_music_downloader
        .get_widevine_license(id, pssh, cdm_license_request.challenge().unwrap())
        .await?;
    let keys = cdm_license_request.get_keys(&license_message).unwrap();
    let key = keys.first_of_type(widevine::KeyType::CONTENT).unwrap();
    let key_hex = hex::encode(key.key.clone());
    Ok(key_hex)
}
