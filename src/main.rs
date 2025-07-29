use std::cmp::Ordering;

use serde::Serialize;
use tokio::net::TcpListener;

mod environments;
mod layers;
mod enemy;
mod api;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3010")
        .await
        .expect("can't listen");
    axum::serve(listener, api::make_router()).await.unwrap();
}

#[derive(Debug)]
pub struct Coins {
    gold: u8,
    silver: u8,
    copper: u8,
}

impl Coins {
    pub fn new<T: AsRef<str>>(g: T, s: T, c: T) -> Self {
        Self {
            gold: g.as_ref().parse::<_>().unwrap(),
            silver: s.as_ref().parse::<_>().unwrap(),
            copper: c.as_ref().parse::<_>().unwrap(),
        }
    }
    pub fn as_copper(&self) -> usize {
        (self.copper as usize
            + (self.silver as usize * 100usize)
            + (self.gold as usize * 1000usize)) as usize
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Unknown,
}

impl Ord for Rarity {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        use Rarity::*;

        match self {
            Common => match other {
                Common => Equal,
                _ => Greater,
            },
            Uncommon => match other {
                Common => Less,
                Uncommon => Equal,
                _ => Greater,
            },
            Rare => match other {
                Common | Uncommon => Less,
                Rare => Equal,
                Unknown => Greater,
            },
            Unknown => match other {
                _ => Less,
            },
        }
    }
}

impl From<&str> for Rarity {
    fn from(value: &str) -> Rarity {
        use Rarity::*;

        match value {
            "Common" => Common,
            "Uncommon" => Uncommon,
            "Rare" => Rare,
            _ => Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum OrderingText {
    Less,
    Equal,
    Greater
}

impl OrderingText {
    pub fn is_eq(&self) -> bool {
        *self == OrderingText::Equal
    }
}

impl From<Ordering> for OrderingText {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Less => Self::Less,
            Ordering::Equal => Self::Equal,
            Ordering::Greater => Self::Greater
        }
    }
}
