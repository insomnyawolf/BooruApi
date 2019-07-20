use crate::reqwest;

use crate::rating::Rating;
use crate::response::BooruResponse;
use crate::server::TargetApi;


use percent_encoding::{percent_encode, utf8_percent_encode, SIMPLE_ENCODE_SET};

#[derive(Debug, Default)]
pub struct BooruRequest {
    pub tags: Option<Vec<String>>, // Done
    pub id: Option<i64>,           // Done
    pub id_before: Option<i64>,    // Done
    pub id_after: Option<i64>,     // Done
    pub random_order: bool,        // Done //Fails on gelboru?
    pub rating: Rating,            // Done
    pub height: Option<i64>,       // Done
    pub width: Option<i64>,        // Done
    pub score: Option<f64>,        // Done
    pub score_min: Option<f64>,    // Done
    pub max_results: Option<i64>,  // Done
    pub target_api: TargetApi,     // Done
    pub server_type: Option<i64>,  // Check If needed
    pub url: Option<String>,       // Check If Needed
}

impl BooruRequest {
    pub fn get_api_query(&self) -> String {
        let mut url: URL = URL::default();

        // Parse Limit
        if let Some(value) = &self.max_results {
            url.add_pair("limit", value.to_string());
        }

        // Check if there's no tags
        let mut tags: Vec<String> = match &self.tags {
            Some(value) => value.clone(),
            None => Vec::new(),
        };

        // Parse Values Used As Tags
        // Parse random_order
        if self.random_order {
            tags.push("order:random".to_owned())
        }

        // parse Rating
        if self.rating != Rating::Any {
            tags.push("rating:".to_owned() + &self.rating.get_rating_string());
        }

        // Parse Score
        if let Some(value) = &self.score {
            tags.push("score:".to_owned() + value.to_string().as_str());
        }

        // Parse Score (min)
        if let Some(value) = &self.score_min {
            tags.push("score:>".to_owned() + value.to_string().as_str());
        }

        // Parse Before
        if let Some(value) = &self.id_before {
            tags.push("id:<".to_owned() + value.to_string().as_str());
        }

        // Parse After
        if let Some(value) = &self.id_after {
            tags.push("id:>".to_owned() + value.to_string().as_str());
        }

        // Parse Id
        if let Some(value) = &self.id {
            tags.push("id:".to_owned() + value.to_string().as_str());
        }

        // Parse Height
        if let Some(value) = &self.height {
            tags.push("height:".to_owned() + value.to_string().as_str());
        }

        // Parse Width
        if let Some(value) = &self.width {
            tags.push("width:".to_owned() + value.to_string().as_str());
        }

        // Parse Tags
        if !tags.is_empty() {
            url.add_pair("tags", tags.join("+"));
        }

        url.url
    }

    pub fn get_api_request(&self) -> String {
        self.target_api.get_api_endpoint() + self.get_api_query().as_str()
    }

    pub fn get_results(&self) -> Option<Vec<BooruResponse>> {
        match reqwest::get(&self.get_api_request()) {
            Ok(mut v) => {
                match v.text(){
                    Ok(data) => {
                        if !data.is_empty() {
                            match serde_json::from_str(&data) {
                                Ok(value) => {
                                    let v: Vec<BooruResponse> = value;
                                    return Some(v);
                                }
                                Err(e) => {
                                    println!("{:?}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                       println!("{:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        None
    }
}

#[derive(Debug, Default)]
pub struct URL {
    pub url: String, // Done
}

impl URL {
    fn add_pair(&mut self, key: &str, value: String) {
        let url_encoded_key =
            percent_encoding::utf8_percent_encode(&key, percent_encoding::DEFAULT_ENCODE_SET)
                .to_string();
        let url_encoded_value =
            percent_encoding::utf8_percent_encode(&value, percent_encoding::DEFAULT_ENCODE_SET)
                .to_string();
        let mut res: String = url_encoded_key.to_owned() + "=";
        res += url_encoded_value.as_str();
        if !self.url.is_empty() {
            self.url += &"&";
        }
        self.url += res.as_str();
    }
}
