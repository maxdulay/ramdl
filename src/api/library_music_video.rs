use super::artwork::Artwork;
use super::editorial_notes::EditorialNotes;
use super::play_parameters::PlayParameters;
use super::previews::Preview;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LibraryMusicVideos {
    /// The identifier for the music video.
    pub id: String,
    /// This value is always music-videos.
    #[serde(rename = "type")]
    pub type_: String,
    /// The relative location for the music video resource.
    pub href: String,
    /// The attributes for the music video.
    pub attributes: Attributes,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    /// The name of the album the music video appears on.
    pub album_name: Option<String>,
    /// The artist’s name.
    pub artist_name: String,
    /// (Extended) The URL of the artist for this content.
    pub artist_url: Option<String>,
    /// The artwork for the music video’s associated album.
    pub artwork: Artwork,
    /// The Recording Industry Association of America (RIAA) rating of the content. No value means no rating.  
    /// Possible Values: clean, explicit
    //pub content_rating: Option<String>,
    /// The duration of the music video in milliseconds.
    pub duration_in_millis: i64,
    /// The music video’s associated genres.
    pub genre_names: Vec<String>,
    /// Whether the music video has 4K content.
    //pub has_4k: bool,
    ///  Whether the music video has HDR10-encoded content.
    //pub has_hdr: bool,
    /// The International Standard Recording Code (ISRC) for the music video.
    //pub isrc: Option<String>,
    /// The localized name of the music video.
    pub name: String,
    /// When present, this attribute indicates that the music video is available to play with an Apple Music subscription.
    /// The value map may be used to initiate playback. Previews of the music video may be available with or without an Apple Music subscription.
    pub play_params: Option<PlayParameters>,
    /// The preview assets for the music video.
    //pub previews: Vec<Preview>,
    /// The release date of the music video, when known, in YYYY-MM-DD or YYYY format.
    /// Prerelease music videos may have an expected release date in the future.
    pub release_date: Option<String>,
    /// The number of the music video in the album’s track list, when associated with an album.
    pub track_number: Option<i64>,
    ///// (Required) The URL for sharing the music video in Apple Music.
    //pub url: String,
    /// The video subtype associated with the content.
    /// (Classical music only) A unique identifier for the associated work.
    pub work_id: Option<String>,
    /// (Classical music only) The name of the associated work.
    pub work_name: Option<String>,
}

