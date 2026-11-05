pub enum GrindLevel {
    ExtraCoarse,
    Coarse,
    MediumCoarse,
    Medium,
    MediumFine,
    Fine,
    ExtraFine,
}

impl GrindLevel {
    pub fn to_sql(&self) -> String {
        match self {
            GrindLevel::ExtraCoarse => String::from("extra-coarse"),
            GrindLevel::Coarse => String::from("coarse"),
            GrindLevel::MediumCoarse => String::from("medium-coarse"),
            GrindLevel::Medium => String::from("medium"),
            GrindLevel::MediumFine => String::from("medium-fine"),
            GrindLevel::Fine => String::from("fine"),
            GrindLevel::ExtraFine => String::from("extra-fine"),
        }
    }
}
