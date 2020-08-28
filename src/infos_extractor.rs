use dbus::{arg, blocking::Connection};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

pub fn milliseconds_to_time(duration: f64) -> String {
    let milliseconds = ((duration % 1000.0) / 100.0).floor();
    let seconds = ((duration / 1000.0) % 60.0).floor();
    let minutes = ((duration / (1000.0 * 60.0)) % 60.0).floor();
    let hours = ((duration / (1000.0 * 60.0 * 60.0)) % 24.0).floor();

    format!(
        "{:0>2}:{:0>2}:{:0>2}:{:0>2}",
        hours, minutes, seconds, milliseconds
    )
}

#[derive(Debug)]
pub struct SongInfos {
    pub artist_image_url: String,
    pub track_number: i64,
    pub disc_number: i64,
    pub track_id: String,
    pub url: String,
    pub album: String,
    pub auto_rating: f64,
    pub total_duration_microseconds: u64,
    pub total_duration_formatted: String,
    pub title: String,
    pub album_artists: Vec<String>,
    pub artists: Vec<String>,
    pub playback_status: String,
}

impl SongInfos {
    fn extract_field<'a>(metadata: &'a Metadata, field: &str) -> &'a dyn arg::RefArg {
        metadata
            .get(field)
            .expect(&format!("Failed to get {}", field))
    }

    fn new(metadata: Metadata, playback_status: String) -> SongInfos {
        let artist_image_url = SongInfos::extract_field(&metadata, "mpris:artUrl")
            .as_str()
            .unwrap()
            .to_owned();
        // Rant: why the fuck should the track number have a negative value?
        let track_number = SongInfos::extract_field(&metadata, "xesam:trackNumber")
            .as_i64()
            .unwrap();
        // Same as above
        let disc_number = SongInfos::extract_field(&metadata, "xesam:discNumber")
            .as_i64()
            .unwrap();
        let track_id = SongInfos::extract_field(&metadata, "mpris:trackid")
            .as_str()
            .unwrap()
            .to_owned();
        let url = SongInfos::extract_field(&metadata, "xesam:url")
            .as_str()
            .unwrap()
            .to_owned();
        let album = SongInfos::extract_field(&metadata, "xesam:album")
            .as_str()
            .unwrap()
            .to_owned();
        let auto_rating = SongInfos::extract_field(&metadata, "xesam:autoRating")
            .as_f64()
            .unwrap();
        let total_duration_microseconds = SongInfos::extract_field(&metadata, "mpris:length")
            .as_u64()
            .unwrap();
        let title = SongInfos::extract_field(&metadata, "xesam:title")
            .as_str()
            .unwrap()
            .to_owned();
        let album_artists = SongInfos::extract_field(&metadata, "xesam:albumArtist")
            .as_iter()
            .unwrap()
            .into_iter()
            .flat_map(|artists| {
                artists
                    .as_iter()
                    .unwrap()
                    .into_iter()
                    .map(|artist| artist.as_str().unwrap().to_owned())
                    .collect::<Vec<String>>()
            })
            .collect();
        let artists = SongInfos::extract_field(&metadata, "xesam:artist")
            .as_iter()
            .unwrap()
            .into_iter()
            .flat_map(|artists| {
                artists
                    .as_iter()
                    .unwrap()
                    .into_iter()
                    .map(|artist| artist.as_str().unwrap().to_owned())
                    .collect::<Vec<String>>()
            })
            .collect();

        SongInfos {
            playback_status,
            artist_image_url,
            track_number,
            disc_number,
            track_id,
            url,
            album,
            auto_rating,
            total_duration_microseconds,
            total_duration_formatted: milliseconds_to_time(
                total_duration_microseconds as f64 / 1000.0,
            ),
            title,
            album_artists,
            artists,
        }
    }
}

type Metadata = HashMap<String, arg::Variant<Box<dyn arg::RefArg>>>;

pub fn current_song_infos() -> Result<SongInfos, Box<dyn Error>> {
    let connection = Connection::new_session()?;
    let process = connection.with_proxy(
        "org.mpris.MediaPlayer2.spotify",
        "/org/mpris/MediaPlayer2",
        Duration::from_millis(5000),
    );

    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

    let metadata: Metadata = process.get("org.mpris.MediaPlayer2.Player", "Metadata")?;

    let playback_status: String = {
        let playback_status: Box<dyn arg::RefArg> =
            process.get("org.mpris.MediaPlayer2.Player", "PlaybackStatus")?;
        playback_status.as_str().unwrap().to_owned()
    };

    Ok(SongInfos::new(metadata, playback_status))
}
