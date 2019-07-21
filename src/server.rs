#[allow(dead_code)]
#[derive(Debug)]
pub enum TargetApi {
    /// konachan.com
    Konachan,
    /// yande.re
    Yandere,
    /// gelbooru.com
    Gelbooru,
    /// rule34.xxx
    Rule34,
    /// safebooru.org
    Safebooru,
    /// tbib.org
    TBIB,
    // This Api Is A Joke
    // Sankaku,
}

impl Default for TargetApi {
    fn default() -> TargetApi {
        TargetApi::Gelbooru
    }
}

impl TargetApi {
    /// Converts the enum value into the server host string
    pub fn get_host(&self) -> String {
        match self {
            TargetApi::Konachan => "https://konachan.com/".to_owned(),
            TargetApi::Yandere => "https://yande.re/".to_owned(),
            TargetApi::Gelbooru => "https://gelbooru.com/".to_owned(),
            TargetApi::Rule34 => "https://rule34.xxx/".to_owned(),
            TargetApi::Safebooru => "https://safebooru.org/".to_owned(),
            TargetApi::TBIB => "https://tbib.org/".to_owned(),
            // TargetApi::Sankaku => "https://capi-beta.sankakucomplex.com/".to_owned(),
        }
    }

    fn get_endpoint(&self) -> String {
        match self {
            TargetApi::Konachan | TargetApi::Yandere => "post.json?".to_owned(),
            TargetApi::Gelbooru | TargetApi::Rule34 | TargetApi::Safebooru | TargetApi::TBIB => {
                "index.php?page=dapi&s=post&q=index&json=1&".to_owned()
            } // TargetApi::Sankaku => "post/index.json?page=1&".to_owned(),
        }
    }

    pub fn get_api_endpoint(&self) -> String {
        self.get_host() + &self.get_endpoint()
    }
}
