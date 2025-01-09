use super::artwork::Artwork;
use super::editorial_notes::EditorialNotes;
use super::play_parameters::PlayParameters;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LibrarySongs {
    /// The identifier for the song.
    pub id: String,
    /// This value is always library-songs.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// The relative location for the song resource.
    pub href: String,
    /// The attributes for the song.
    pub attributes: Attributes,
    ///// The relationships for the library music vide.
    //pub relationships: Option<Relationships>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    /// The name of the album the song appears on.
    pub album_name: Option<String>,
    /// The artist’s name.
    pub artist_name: String,
    /// The URL of the artist for the content.
    pub artist_url: Option<String>,
    /// The album artwork.
    pub artwork: Artwork,
    /// (Classical music only) The name of the artist or composer to attribute the song with.
    pub attribution: Option<String>,
    /// (Extended) Indicates the specific audio variant for a song.
    /// Possible Values: dolby-atmos, dolby-audio, hi-res-lossless, lossless, lossy-stereo
    /// ## Important
    /// Use badges to indicate the audio variant in Apple Music. For more information, see Dolby Asset Center (DAC) for identifying tracks with the Dolby Atmos logo and Apple Lossless glyphs for identifying tracks with the specific Apple Lossless glyph.
    pub audio_variants: Option<Vec<String>>,
    /// The song’s composer.
    pub composer_name: Option<String>,
    /// The Recording Industry Association of America (RIAA) rating of the content. The possible values for this rating are clean and explicit. No value means no rating.
    pub content_rating: Option<String>,
    /// The disc number the song appears on.
    pub disc_number: u32,
    /// The approximate length of the song in milliseconds.
    pub duration_in_millis: u64,
    /// The notes about the song that appear in the Apple Music catalog.
    pub editorial_notes: Option<EditorialNotes>,
    /// The genre names the song is associated with.
    pub genre_names: Vec<String>,
    /// Indicates whether the song has lyrics available in the Apple Music catalog. If true, the song has lyrics available; otherwise, it doesn't.
    pub has_lyrics: bool,
    /// (Classical music only) The movement count of the song.
    pub movement_count: Option<i32>,
    /// (Classical music only) The movement name of the song.
    pub movement_name: Option<String>,
    /// (Classical music only) The movement number of the song.
    pub movement_number: Option<i32>,
    /// The localized name of the song.
    pub name: String,
    /// When present, this attribute indicates that the song is available to play with an Apple Music subscription. The value map may be used to initiate playback. Previews of the song audio may be available with or without an Apple Music subscription.
    pub play_params: Option<PlayParameters>,
    /// Day of release
    pub release_data: Option<String>,
    pub track_number: u32,
}


