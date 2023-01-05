use std::collections::HashSet;

use rspotify::scopes;

pub fn get_scopes() -> HashSet<String> {
    scopes!(
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-read-playback-position",
        "user-modify-playback-state",
        "user-library-read"
    )
}
