#[derive(Clone, Copy, Debug)]
enum Sex {
    Female,
    Male,
}

impl Sex {
    fn slug(&self) -> &'static str {
        match self {
            Sex::Female => "female",
            Sex::Male => "male",
        }
    }

    fn all() -> &'static [Sex] {
        &[Sex::Female, Sex::Male]
    }
}

#[derive(Clone, Copy, Debug)]
enum Age {
    YoungAdult,
    Adult,
    Child,
    Elderly,
    Infant,
}

impl Age {
    fn slug(&self) -> &'static str {
        match self {
            Age::YoungAdult => "young-adult",
            Age::Adult => "adult",
            Age::Child => "child",
            Age::Elderly => "elderly",
            Age::Infant => "infant",
        }
    }

    fn all() -> &'static [Age] {
        &[
            Age::YoungAdult,
            Age::Adult,
            Age::Child,
            Age::Elderly,
            Age::Infant,
        ]
    }
}

#[derive(Clone, Copy, Debug)]
enum Ethnicity {
    Asian,
    Black,
    Latino,
    White,
}

impl Ethnicity {
    fn slug(&self) -> &'static str {
        match self {
            Ethnicity::Asian => "asian-race",
            Ethnicity::Black => "black-race",
            Ethnicity::Latino => "latino-race",
            Ethnicity::White => "white-race",
        }
    }

    fn all() -> &'static [Ethnicity] {
        &[
            Ethnicity::Asian,
            Ethnicity::Black,
            Ethnicity::Latino,
            Ethnicity::White,
        ]
    }
}

#[derive(Clone, Copy, Debug)]
enum EyeColor {
    Brown,
    Grey,
    Blue,
    Green,
}

impl EyeColor {
    fn slug(&self) -> &'static str {
        match self {
            EyeColor::Brown => "brown-eyes",
            EyeColor::Grey => "grey-eyes",
            EyeColor::Blue => "blue-eyes",
            EyeColor::Green => "green-eyes",
        }
    }

    fn all() -> &'static [EyeColor] {
        &[
            EyeColor::Brown,
            EyeColor::Grey,
            EyeColor::Blue,
            EyeColor::Green,
        ]
    }
}

#[derive(Clone, Copy, Debug)]
enum HairColor {
    Brown,
    Black,
    Blond,
    Gray,
}

impl HairColor {
    fn slug(&self) -> &'static str {
        match self {
            HairColor::Brown => "brown-hair",
            HairColor::Black => "black-hair",
            HairColor::Blond => "blond-hair",
            HairColor::Gray => "gray-hair",
        }
    }

    fn all() -> &'static [HairColor] {
        &[
            HairColor::Brown,
            HairColor::Black,
            HairColor::Blond,
            HairColor::Gray,
        ]
    }
}

#[derive(Clone, Copy, Debug)]
enum HairLength {
    Short,
    Medium,
    Long,
}

impl HairLength {
    fn slug(&self) -> &'static str {
        match self {
            HairLength::Short => "short",
            HairLength::Medium => "medium",
            HairLength::Long => "long",
        }
    }

    fn all() -> &'static [HairLength] {
        &[HairLength::Short, HairLength::Medium, HairLength::Long]
    }
}
