use crate::environments::{Environment, EnvironmentDiff};
use crate::layers::{Layer, LayerDiff};
use crate::{Coins, Rarity, OrderingText};
use rand::Rng;
use serde::Serialize;

const ENEMIES: &'static str = include_str!("./data/enemies");

pub fn get_random_enemy() -> Enemy {
    let lines = ENEMIES.lines().skip(1).collect::<Vec<&str>>();
    let max = lines.len();
    let mut rng = rand::rng();

    let line_idx = rng.random_range(0..max);
    let line = lines.get(line_idx).unwrap();
    let enemy = Enemy::from(*line);

    println!("The daily enemy is:\n{enemy:?}");
    enemy
}

pub fn get_all_enemies() -> Vec<Enemy> {
    ENEMIES.lines().skip(1).map(Enemy::from).collect()
}

#[derive(Debug, Serialize)]
pub struct EnemyDiff {
    name: bool,
    life: OrderingText,
    defence: OrderingText,
    coins: OrderingText,
    environments: EnvironmentDiff,
    layers: LayerDiff,
    rarity: OrderingText,
}

impl EnemyDiff {
    /// Whether or not this diff was the result of two of the same enemy.
    pub fn is_same(&self) -> bool {
        self.name
            && self.life.is_eq()
            && self.defence.is_eq()
            && self.coins.is_eq()
            && self.environments.wrong.is_empty()
            && self.layers.wrong.is_empty()
    }
}

#[derive(Debug)]
pub struct Enemy {
    pub id: u16,
    pub name: Box<str>,
    pub life: u16,
    pub defence: u16,
    pub coins: Coins,
    pub environments: Vec<Environment>,
    pub layers: Vec<Layer>,
    pub rarity: Rarity,
}

impl Enemy {
    pub fn diff(&self, other: &Enemy) -> EnemyDiff {
        let name = self.name == other.name;
        let life = self.life.cmp(&other.life).into();
        let defence = self.defence.cmp(&other.defence).into();
        let coins = self.coins.as_copper().cmp(&other.coins.as_copper()).into();
        let environments = self.diff_env(&other.environments);
        let layers = self.diff_layer(&other.layers);
        let rarity = self.rarity.cmp(&other.rarity).into();

        EnemyDiff {
            name,
            life,
            defence,
            coins,
            environments,
            layers,
            rarity,
        }
    }
    pub fn diff_env(&self, other: &[Environment]) -> EnvironmentDiff {
        let (right, wrong): (Vec<Environment>, Vec<Environment>) = self
            .environments
            .iter()
            .cloned()
            .partition(|env| other.contains(env));
        EnvironmentDiff { right, wrong }
    }
    pub fn diff_layer(&self, other: &[Layer]) -> LayerDiff {
        let (right, wrong): (Vec<Layer>, Vec<Layer>) = self
            .layers
            .iter()
            .cloned()
            .partition(|lay| other.contains(lay));
        LayerDiff { right, wrong }
    }
}

impl From<&str> for Enemy {
    fn from(value: &str) -> Self {
        let mut chunks = value.split(',');
        let id = chunks.next().unwrap();
        let name = chunks.next().unwrap();
        let life = chunks.next().unwrap();
        let defence = chunks.next().unwrap();
        let g = chunks.next().unwrap();
        let s = chunks.next().unwrap();
        let c = chunks.next().unwrap();
        let environment = chunks.next().unwrap();
        let layer = chunks.next().unwrap();
        let rarity = chunks.next().expect(&format!("{id}"));

        Self {
            id: id.parse::<_>().unwrap(),
            name: name.to_owned().into_boxed_str(),
            life: life.parse::<_>().unwrap(),
            defence: defence.parse::<_>().unwrap(),
            coins: Coins::new(g, s, c),
            environments: environment.split('/').map(Environment::from).collect(),
            layers: layer.split('/').map(Layer::from).collect(),
            rarity: rarity.try_into().unwrap(),
        }
    }
}
