#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Rating {
    Any,
    Safe,
    QuestionablePG,
    Questionable,
    QuestionableExplicit,
    Explicit,
}

impl Default for Rating {
    fn default() -> Rating {
        Rating::Any
    }
}

impl Rating {
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
