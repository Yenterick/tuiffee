#[derive(Eq, PartialEq)]
pub enum Pages {
    Home,
    CoffeeVarieties,
    CupCoffee,
    PreparationMethods,
    PrepareCoffee,
}

impl Pages {
    pub fn to_string(&self) -> String {
        match self {
            Pages::Home => String::from("Home"),
            Pages::CoffeeVarieties => String::from("Coffee Varieties"),
            Pages::CupCoffee => String::from("Cup Coffee"),
            Pages::PreparationMethods => String::from("Preparation Methods"),
            Pages::PrepareCoffee => String::from("Prepare Coffee"),
        }
    }
}
