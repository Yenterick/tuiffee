pub enum ProcessType {
    Washed,
    Honey,
    Natural,
    Culturing,
}

impl ProcessType {
    pub fn to_sql(&self) -> String {
        match self {
            ProcessType::Washed => String::from("washed"),
            ProcessType::Honey => String::from("honey"),
            ProcessType::Natural => String::from("natural"),
            ProcessType::Culturing => String::from("culturing"),
        }
    }
}
