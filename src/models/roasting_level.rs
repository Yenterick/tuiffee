pub enum RoastingLevel {
    Light,
    Medium,
    MediumDark,
    Dark,
}

impl RoastingLevel {
    pub fn to_sql(&self) -> String {
        match self {
            RoastingLevel::Light => String::from("light"),
            RoastingLevel::Medium => String::from("medium"),
            RoastingLevel::MediumDark => String::from("medium-dark"),
            RoastingLevel::Dark => String::from("dark"),
        }
    }
}
