// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{borrow::Cow, sync::Mutex};

use serde::Serialize;
use tauri::State;

#[derive(Serialize, Debug)]
struct Item {
    item: String,
    item_type: String,
    series: String,
    certification: String,
    quality: String,
    paint: String,
    search_type: String,
    platform: String,
}

impl Item {
    pub fn from_link(link: &url::Url) -> Self {
        let params = link.query_pairs();

        let item = params
            .clone()
            .find(|(name, _)| name == "filterItem")
            .unwrap()
            .1
            .to_string();

        let certification = params
            .clone()
            .find(|(name, _)| name == "filterCertification")
            .unwrap_or((Cow::default(), Cow::from("0")))
            .1
            .to_string();

        let item_type = params
            .clone()
            .find(|(name, _)| name == "filterItemType")
            .unwrap_or((Cow::default(), Cow::from("0")))
            .1
            .to_string();

        let quality = params
            .clone()
            .find(|(name, _)| name == "filterRarity")
            .unwrap_or((Cow::default(), Cow::from("0")))
            .1
            .to_string();

        let series = params
            .clone()
            .find(|(name, _)| name == "filterSeries")
            .unwrap_or((Cow::default(), Cow::from("0")))
            .1
            .to_string();

        let paint = params
            .clone()
            .find(|(name, _)| name == "filterPaint")
            .unwrap_or((Cow::default(), Cow::from("0")))
            .1
            .to_string();

        let search_type = params
            .clone()
            .find(|(name, _)| name == "filterSearchType")
            .unwrap_or((Cow::default(), Cow::from("0")))
            .1
            .to_string();

        let platform = params
            .clone()
            .find(|(name, _)| name == "filterPlatform")
            .unwrap_or((Cow::default(), Cow::from("0")))
            .1
            .to_string();

        Self {
            item,
            certification,
            item_type,
            quality,
            series,
            paint,
            search_type,
            platform,
        }
    }
}

#[tauri::command]
fn parse(link: String) -> i32 {
    10
}

#[tauri::command]
fn links_to_cells(links: Vec<String>, assets: State<AssetsState>) -> Vec<Item> {
    let assets = assets.0.lock().unwrap();

    let cells = links
        .iter()
        .map(|link| {
            let url = url::Url::parse(link).unwrap();
            Item::from_link(&url)
        })
        .map(|cell| {
            let item = match assets.items.get(&cell.item) {
                Some(item) => item.name.to_owned(),
                None => "неизвестный предмет".to_owned(),
            };

            let paint = assets
                .paints
                .get(&cell.paint)
                .unwrap_or(&"none".to_owned())
                .to_owned();

            let quality = assets
                .qualities
                .get(&cell.quality)
                .unwrap_or(&"none".to_owned())
                .to_owned();

            let certification = assets
                .certifications
                .get(&cell.certification)
                .unwrap_or(&"none".to_owned())
                .to_owned();

            let series = assets
                .series
                .get(&cell.series)
                .unwrap_or(&"none".to_owned())
                .to_owned();

            let item_type = assets
                .item_types
                .get(&cell.item_type)
                .unwrap_or(&"none".to_owned())
                .to_owned();

            let search_type = assets
                .search_types
                .get(&cell.search_type)
                .unwrap_or(&"none".to_owned())
                .to_owned();

            let platform = assets
                .platforms
                .get(&cell.platform)
                .unwrap_or(&"none".to_owned())
                .to_owned();

            Item {
                item,
                item_type,
                series,
                certification,
                quality,
                paint,
                search_type,
                platform,
            }
        })
        .collect();

    cells
}

struct AssetsState(Mutex<assets::assets::Assets>);

#[tokio::main]
async fn main() {
    let assets = assets::assets::Assets::new().await.unwrap();

    tauri::Builder::default()
        .manage(AssetsState(Mutex::new(assets)))
        .invoke_handler(tauri::generate_handler![links_to_cells, parse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
