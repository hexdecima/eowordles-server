use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub(crate) enum Environment {
    Any,
    Forest,
    Snow,
    Jungle,
    Desert,
    Ocean,
    Corruption,
    Crimson,
    Hallow,
    Space,
    Mushroom,
    Dungeon,
    Event,
    Day,
    Night,
    Graveyard,
    Goblin,
    Pirate,
    Rain,
    Martian,
    Eclipse,
    OldOnes,
    Blood,
    Lunar,
    Unknown,
}

impl From<&str> for Environment {
    fn from(value: &str) -> Self {
        use Environment::*;

        match value {
            "Any" => Any,
            "Forest" => Forest,
            "Snow" => Snow,
            "Jungle" => Jungle,
            "Desert" => Desert,
            "Ocean" => Ocean,
            "Corruption" => Corruption,
            "Crimson" => Crimson,
            "Hallow" => Hallow,
            "Space" => Space,
            "Mushroom" => Mushroom,
            "Dungeon" => Dungeon,
            "Graveyard" => Graveyard,
            "Day" => Day,
            "Goblin" => Goblin,
            "Pirate" => Pirate,
            "Night" => Night,
            "Event" => Event,
            "OldOnes" => OldOnes,
            "Blood" => Blood,
            "Martian" => Martian,
            "Eclipse" => Eclipse,
            "Lunar" => Lunar,
            "Rain" => Rain,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct EnvironmentDiff {
    pub right: Vec<Environment>,
    pub wrong: Vec<Environment>,
}
