#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Rating {
    /// Any kind of results
    Any,
    /// Only Safe For Work Pictures
    Safe,
    /// Safe For Work and Ecchi Pictures
    /// May not work on every server
    QuestionablePG,
    /// Only Ecchi pictures
    Questionable,
    /// Ecchi and explicit pictures
    /// May not work on every server
    QuestionableExplicit,
    /// Only Explicit Pictures
    Explicit,
}

impl Default for Rating {
    fn default() -> Rating {
        Rating::Any
    }
}

impl Rating {
    /// Converts the value into the api equivalent value
    pub fn get_rating_string(&self) -> String {
        match self {
            Rating::Any => "".to_owned(),
            Rating::Safe => "safe".to_owned(),
            Rating::QuestionablePG => "questionableless".to_owned(),
            Rating::Questionable => "questionable".to_owned(),
            Rating::QuestionableExplicit => "questionableplus".to_owned(),
            Rating::Explicit => "explicit".to_owned(),
        }
    }
}
