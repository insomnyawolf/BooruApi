use crate::reqwest;

use crate::rating::Rating;
use crate::response::BooruResponse;
use crate::server::TargetApi;

#[derive(Debug, Default)]
pub struct BooruRequest {
    /// I think the name is autodescriptive enought on this
    /// ¯\_(ツ)_/¯
    pub tags: Option<Vec<String>>, // Done
    /// Retrieve post by id
    pub id: Option<i64>,           // Done
    /// Ask api for post before specific id
    pub id_before: Option<i64>,    // Done
    /// Ask api for post after specific id
    pub id_after: Option<i64>,     // Done
    /// Request a randomized post including the other tags
    /// Works Only On Moebooru Based Servers
    ///
    /// * Konachan
    /// * Yandere
    ///
    /// Fails on
    ///
    /// * Gelbooru
    /// * TBIB
    /// * Rule34
    /// * Safebooru
    ///
    pub random_order: bool,        // Done
    /// Image Rating, May be:
    ///
    /// * Any
    /// * Safe
    /// * QuestionablePG
    /// * Questionable
    /// * QuestionableExplicit
    /// * Explicit
    ///
    pub rating: Rating,            // Done
    /// Height Image Dimensions
    /// By default it will be Any
    pub height: Option<i64>,       // Done
    /// Width Image Dimensions
    pub width: Option<i64>,        // Done
    /// Ask the api for images with a specific score
    pub score: Option<f64>,        // Done
    /// Ask the api for images above the score value
    pub score_min: Option<f64>,    // Done
    /// Defines how much post should request to the api
    pub max_results: Option<i64>,  // Done
    /// Wich api will be used in the request
    /// By default Gelbooru
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
