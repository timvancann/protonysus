use id3::{Tag as Id3Tag, TagLike};
use log::info;
use crate::player::{Id3Tags, Track};
use base64::prelude::*;
use id3::frame::PictureType::CoverFront;
use image::imageops::resize;
use metaflac::{Tag as FlacTag};


pub fn extract_id3_tags(track: &mut Track) {
    // uses the ID3 library to extact tags from the track
    // and updates the track with the extracted tags
    // the updated track is then returned

    let tag = Id3Tag::read_from_path(&track.path);
    if let Ok(t) = tag {
        track.id3_tags.title = t.title().map(|s| s.to_string());
        track.id3_tags.artist = t.artist().map(|s| s.to_string());
        track.id3_tags.album = t.album().map(|s| s.to_string());
        track.id3_tags.year = t.year().map(|s| s.to_string());
        track.id3_tags.album_art = t.pictures()
            .next()
            .map(|p| p.data.to_vec())
            // .map(|arr| resize(&image::load_from_memory(&arr).unwrap(), 100, 100, image::imageops::FilterType::Nearest).to_vec())
            .map(|arr| format!("data:image/jpeg;base64,{}", BASE64_STANDARD.encode(arr)));
        track.id3_tags.track = t.track();
    }
}

pub fn extract_flac_tags(track: &mut Track) {
    let tag = FlacTag::read_from_path(&track.path);
    if let Ok(tag) = tag {
        track.id3_tags.title = tag.get_vorbis("TITLE").and_then(|s| s.into_iter().next()).map(|s| s.to_string());
        track.id3_tags.artist = tag.get_vorbis("ARTIST").and_then(|s| s.into_iter().next()).map(|s| s.to_string());
        track.id3_tags.album = tag.get_vorbis("ALBUM").and_then(|s| s.into_iter().next()).map(|s| s.to_string());
        track.id3_tags.year = tag.get_vorbis("DATE").and_then(|s| s.into_iter().next()).map(|s| s.to_string());
        track.id3_tags.album_art = tag.pictures()
            .next()
            .map(|p| p.data.to_vec())
            .map(|arr| format!("data:image/jpeg;base64,{}", BASE64_STANDARD.encode(arr)));
    };
}