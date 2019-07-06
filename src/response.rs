use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BooruResponse {
    //
    // Konachan Fields
    //
    id: Option<i64>,
    tags: Option<String>,
    created_at: Option<i64>,
    creator_id: Option<i64>,
    author: Option<String>,
    change: Option<i64>,
    source: Option<String>,
    score: Option<i64>,
    md5: Option<String>,
    file_size: Option<i64>,
    file_url: Option<String>,
    is_shown_in_index: Option<bool>,
    preview_url: Option<String>,
    preview_width: Option<i64>,
    preview_height: Option<i64>,
    actual_preview_width: Option<i64>,
    actual_preview_height: Option<i64>,
    sample_url: Option<String>,
    sample_width: Option<i64>,
    sample_height: Option<i64>,
    sample_file_size: Option<i64>,
    jpeg_url: Option<String>,
    jpeg_width: Option<i64>,
    jpeg_height: Option<i64>,
    jpeg_file_size: Option<i64>,
    rating: Option<String>,
    has_children: Option<bool>,
    parent_id: Option<String>,
    status: Option<String>,
    width: Option<i64>,
    height: Option<i64>,
    is_held: Option<bool>,
    frames_pending_string: Option<String>,
    frames_string: Option<String>,
    //
    // Is this even used?
    //
    // frames_pending: Vec<>,
    // frames: Vec<>,
    //
    // ##########################
    //
    // Gelboru Fields
    //
    directory: Option<String>,
    hash: Option<String>,
    image: Option<String>,
    owner: Option<String>,
    sample: Option<bool>,
}
