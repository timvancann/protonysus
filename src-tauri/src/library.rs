fn modify_lirary(library: &mut Library, files: Vec<&str>) -> Library {
    let mut start = Instant::now();
    let new_tracks = scan_files(files).par_iter().map(file_to_track)
        .flatten()
        .collect();
    let mut all_tracks = dedup_tracks(&library.lock().unwrap().tracks, &new_tracks);
    all_tracks.sort_by_key(|track| (track.id3_tags.artist.clone(), track.id3_tags.album.clone(), track.id3_tags.track, track.id3_tags.title.clone()));

    let mut lib = library.lock().unwrap();
    lib.tracks = all_tracks;

    println!("Scanned {} files in {:?}", new_tracks.len(), start.elapsed());


fn scan_files(paths: Vec<&str>) -> Vec<PathBuf> {
    // recursively scan directories for files and return af flattened list of files found
    paths.iter().flat_map(|path| {
        let path = PathBuf::from(path);
        if path.is_dir() {
            walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().is_file())
                .map(|entry| entry.path().to_path_buf())
                .collect::<Vec<PathBuf>>()
        } else {
            vec![path]
        }
    })
        .filter(|path| {
            path.extension().map_or(false, |ext| {
                let accepted = ACCEPTED_EXTENSIONS.contains(&ext.to_str().unwrap());
                if !accepted {
                    eprintln!("File {:?} has an unsupported extension", path);
                }
                accepted
            })
        })
        .collect()

}
fn dedup_tracks(existing_tracks: &Vec<Track>, new_tracks: &Vec<Track>) -> Vec<Track> {
    existing_tracks
        .iter()
        .chain(new_tracks.iter())
        .collect::<HashSet<_>>()
        .into_iter().cloned()
        .collect::<Vec<Track>>()
}

fn file_to_track(buf: &PathBuf) -> Option<Track> {

    let file = match std::fs::File::open(buf) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {:?}: {}", buf.to_str(), e);
            return None
        }
    };
    let source = match Decoder::new(BufReader::new(file)) {
        Ok(source) => source,
        Err(e) => {
            eprintln!("Error decoding file {:?}: {}", buf.to_str(), e);
            return None
        }
    };
    let mut track = Track {
        path: buf.clone(),
        file_name: buf.file_name().unwrap().to_str().unwrap().to_string(),
        duration: source.total_duration().unwrap_or(Duration::from_millis(0)).as_millis() as u64,
        id3_tags: Id3Tags::new(),
    };
    extract_id3_tags(&mut track);
    extract_flac_tags(&mut track);
    Some(track)
}