
#[derive(Clone, Copy)]
pub enum Space {
    X,
    Y,
    EMPTY
}

impl Space {
    pub fn string_form(&self) -> String{
        match self {
            Self::X => "X".to_string(),
            Self::Y => "Y".to_string(),
            Self::EMPTY => " ".to_string(),
        }
    }
}

impl PartialEq for Space {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::X => match other { Self::X => true, _ => false},
            Self::Y => match other { Self::Y => true, _ => false},
            Self::EMPTY => match other { Self::EMPTY => true, _ => false},
        }
    }
}
