#[derive(Clone, PartialEq, Debug)]
pub struct Player {
    pub name: String,
    pub chips: i64,
}

impl Player {
    pub fn from_name(name: String) -> Self {
        Self { name, chips: 5 }
    }
}

