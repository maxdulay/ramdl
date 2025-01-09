#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LibraryArtists {
    pub id: String, 
    #[serde(rename = "type")]
    pub type_: String,
    pub href: String,
    pub attributes: Attributes,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub name: String,
}
