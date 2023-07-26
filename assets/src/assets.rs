use crate::parser;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs};
use tokio::runtime::Runtime;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ItemCategories {
    Miscellaneous,
    Bodies,
    Wheels,
    Decals,
    RocketBoosts,
    GoalExplosions,
    Toppers,
    PaintFinishes,
    Antennas,
    Trails,
    Banners,
    EngineAudio,
    AvatarBorders,
}

impl ToString for ItemCategories {
    fn to_string(&self) -> String {
        match self {
            ItemCategories::Miscellaneous => "Miscellaneous".to_owned(),
            ItemCategories::Bodies => "Bodies".to_owned(),
            ItemCategories::Wheels => "Wheels".to_owned(),
            ItemCategories::Decals => "Decals".to_owned(),
            ItemCategories::RocketBoosts => "Rocket boosts".to_owned(),
            ItemCategories::GoalExplosions => "Goal explosions".to_owned(),
            ItemCategories::Toppers => "Topper".to_owned(),
            ItemCategories::PaintFinishes => "Paint finishes".to_owned(),
            ItemCategories::Antennas => "Antennas".to_owned(),
            ItemCategories::Trails => "Trails".to_owned(),
            ItemCategories::Banners => "Banners".to_owned(),
            ItemCategories::EngineAudio => "Engine audio".to_owned(),
            ItemCategories::AvatarBorders => "Avatar border".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub series: Vec<u8>,
    pub rarities: Vec<u8>,
    pub category: String,
}

pub type Asset = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Assets {
    pub items: HashMap<String, Item>,
    pub certifications: Asset,
    pub paints: Asset,
    pub series: Asset,
    pub qualities: Asset,
    pub platforms: Asset,
    pub search_types: Asset,
    pub item_types: Asset,
}

impl Assets {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let assets = Self::read_files().unwrap_or_else(|_| {
            let parsed = Runtime::new().unwrap().block_on(parser::parse()).unwrap();
            parsed
        });

        Ok(assets)
    }

    fn read_files() -> Result<Self, Box<dyn Error>> {
        let items = fs::read_to_string("parsed/items.json")?;
        let certifications = fs::read_to_string("parsed/certifications.json")?;
        let paints = fs::read_to_string("parsed/paints.json")?;
        let series = fs::read_to_string("parsed/series.json")?;
        let qualities = fs::read_to_string("parsed/qualities.json")?;
        let platforms = fs::read_to_string("parsed/platforms.json")?;
        let search_types = fs::read_to_string("parsed/search_types.json")?;
        let item_types = fs::read_to_string("parsed/item_types.json")?;

        Ok(Assets {
            items: serde_json::from_str(&items)?,
            certifications: serde_json::from_str(&certifications)?,
            paints: serde_json::from_str(&paints)?,
            series: serde_json::from_str(&series)?,
            qualities: serde_json::from_str(&qualities)?,
            platforms: serde_json::from_str(&platforms)?,
            search_types: serde_json::from_str(&search_types)?,
            item_types: serde_json::from_str(&item_types)?,
        })
    }
}
