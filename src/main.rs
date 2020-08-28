use lazy_static::lazy_static;
use regex::{Captures, Regex};
use spotify_infos::SongInfos;
use std::{env, process};

fn access_song_infos(song_infos: &SongInfos, field: &str) -> Option<String> {
    match field {
        "art_image_url" => Some(song_infos.artist_image_url.clone()),
        "track_number" => Some(song_infos.track_number.to_string()),
        "disc_number" => Some(song_infos.disc_number.to_string()),
        "track_id" => Some(song_infos.track_id.clone()),
        "url" => Some(song_infos.url.clone()),
        "album" => Some(song_infos.album.clone()),
        "auto_rating" => Some(song_infos.auto_rating.to_string()),
        "total_duration_microseconds" => Some(song_infos.total_duration_microseconds.to_string()),
        "total_duration_formatted" => Some(song_infos.total_duration_formatted.clone()),
        "title" => Some(song_infos.title.clone()),
        "album_artists" => Some(song_infos.album_artists.join(" feat. ")),
        "artists" => Some(song_infos.artists.join(" feat. ")),
        "playback_status" => Some(song_infos.playback_status.clone()),
        _ => None,
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"%([^%]+)%").unwrap();
}

fn main() {
    let song_infos =
        spotify_infos::current_song_infos().expect("Failed to fetch current song infos");
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        println!("{:#?}", song_infos);
        process::exit(0);
    }

    if args.len() != 2 {
        eprintln!("Error: you must give exactly one argument, the format string:");
        eprintln!("       ./spotify-infos '%artists% - %title%'");
        process::exit(1);
    }
    let format_str = &args[1];

    let output = RE.replace_all(format_str, |caps: &Captures| {
        access_song_infos(&song_infos, &caps[1])
            .unwrap_or_else(|| panic!("Invalid format tag: `{}`", &caps[1]))
    });
    println!("{}", output);
}
