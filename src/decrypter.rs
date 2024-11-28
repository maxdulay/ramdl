use crate::{error::Result, stream_info::StreamInfo, AppleMusicDownloader};
use base64::Engine;
use pssh_box::{widevine::WidevinePsshData, PsshBox, PsshData, ToBytes};
use rust_widevine::{LicenseDecryptionModule, Session};

pub struct Decrypter {
    pub ldm: LicenseDecryptionModule,
}

impl Decrypter {
    pub fn new(device_private_key: Vec<u8>, device_client_id_blob: Vec<u8>) -> Self {
        let ldm = LicenseDecryptionModule::new(&device_private_key, device_client_id_blob);
        Self { ldm }
    }
    pub async fn get_license(
        &mut self,
        pssh: &str,
        id: &str,
        apple_music: &AppleMusicDownloader,
        session: &mut Session,
    ) -> Result<Vec<u8>> {
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
        let pssh_obj = pssh_box.to_bytes();
        let challenge = session.create_license_request(&self.ldm, pssh_obj).unwrap();

        let license = apple_music
            .get_widevine_license(id, pssh, challenge)
            .await?;
        Ok(license)
    }

    pub async fn get_decrypt_key(
        &mut self,
        stream_info: &StreamInfo,
        apple_music_downloader: &AppleMusicDownloader,
    ) -> Result<String> {
        let mut session = Session::new();
        let license = self
            .get_license(
                &stream_info.pssh,
                "1753050648",
                &apple_music_downloader,
                &mut session,
            )
            .await
            .unwrap();
        let key_containers = session.parse_license(&self.ldm, license).unwrap();
        let key_container = key_containers
            .iter()
            .find(|key_container| key_container.key.len() == 32)
            .unwrap();
        let key = key_container.key.clone();
        Ok(key)
    }
}
