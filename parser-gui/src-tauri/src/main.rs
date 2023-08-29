// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{borrow::Cow, sync::Mutex};

use parser::parser;
use serde::Serialize;
use tauri::State;
use url::Url;

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

const IGNORE: [&'static str; 32] = [
    "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "c10", "c11", "c12", "c13", "c14",
    "1709", "1701", "1708", "1707", "1710", "3007", "2999", "1705", "1703", "3000", "544", "3746",
    "3745", "3744", "3628", "1704", "1706", "2615",
];

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
            .unwrap_or((Cow::default(), Cow::from("A")))
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
            .find(|(name, _)| name == "filterPlatform[]")
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

#[derive(Debug, serde::Serialize)]
struct Price {
    pub id: String,
    pub username: String,
    pub price: usize,
    pub time: String,
}

#[tauri::command]
async fn parse(link: String) -> Vec<Price> {
    // Build url
    let url = Url::parse(&link).unwrap();

    let link = Url::parse_with_params(&link, &[("filterTradeType", "2")]).unwrap();

    let item = Item::from_link(&url);
    let parsed = parser::Trade::parse_many(link.as_str()).await.unwrap();

    let mut filtered: Vec<Price> = vec![];

    for trade in parsed {
        if item.platform != "0" && trade.platform != item.platform {
            continue;
        }

        if item.search_type == "1" {
            // Find in has
            for (index, cell) in trade.has.iter().enumerate() {
                if cell.item == item.item
                    && (cell.paint == item.paint || (cell.paint == "0" && item.paint == "N"))
                    && (cell.quality == item.quality || item.quality == "A")
                    && (cell.certification == item.certification
                        || item.certification == "0"
                        || item.certification == "A"
                        || (cell.certification == "0" && item.certification == "N"))
                    && (cell.series == item.series
                        || item.series == "A"
                        || (cell.series == "0" && item.series == "N"))
                    && (cell.item_type == item.item_type
                        || (item.item_type == "0" && cell.item_type == "1"))
                {
                    let price = trade.wants.get(index);

                    if price.is_none() {
                        break;
                    }

                    let price = price.unwrap().count;

                    filtered.push(Price {
                        id: trade.id,
                        username: trade.username,
                        price,
                        time: trade.time,
                    });

                    break;
                }
            }
        } else {
            // Find in wants
            for (index, cell) in trade.wants.iter().enumerate() {
                if cell.item == item.item
                    && (cell.paint == item.paint || (cell.paint == "0" && item.paint == "N"))
                    && (cell.quality == item.quality || item.quality == "A")
                    && (cell.certification == item.certification
                        || item.certification == "0"
                        || item.certification == "A"
                        || (cell.certification == "0" && item.certification == "N"))
                    && (cell.series == item.series
                        || item.series == "A"
                        || (cell.series == "0" && item.series == "N"))
                    && (cell.item_type == item.item_type
                        || (item.item_type == "0" && cell.item_type == "1"))
                {
                    let price = trade.has.get(index);

                    if price.is_none() {
                        break;
                    }

                    let price = price.unwrap().count;

                    filtered.push(Price {
                        id: trade.id,
                        username: trade.username,
                        price,
                        time: trade.time,
                    });

                    break;
                }
            }
        }
    }

    filtered
}

#[tauri::command]
async fn parse_trade_links(link: String) -> Vec<String> {
    let trade = parser::Trade::parse_one(&link).await.unwrap();
    let mut has_cells: Vec<String> = trade
        .has
        .iter()
        .filter(|cell| !IGNORE.contains(&cell.item.as_str())).map(|cell| {
            format!("https://rocket-league.com/trading?filterItem={}&filterCertification={}&filterPaint={}&filterSeries={}&filterRarity={}&filterTradeType=2&filterSearchType=1&filterItemType={}&filterPlatform[]={}", cell.item, cell.certification, cell.paint, cell.series, cell.quality,cell.item_type, trade.platform)
        }).collect();

    let mut wants_cells: Vec<String> = trade
        .wants
        .iter()
        .filter(|cell| !IGNORE.contains(&cell.item.as_str())).map(|cell| {
            format!("https://rocket-league.com/trading?filterItem={}&filterCertification={}&filterPaint={}&filterSeries={}&filterRarity={}&filterTradeType=2&filterSearchType=2&filterItemType={}&filterPlatform[]={}", cell.item, cell.certification, cell.paint, cell.series, cell.quality,cell.item_type, trade.platform)
        }).collect();

    has_cells.append(&mut wants_cells);

    has_cells
}

#[tauri::command]
async fn parse_profile_links(link: String) -> Vec<String> {
    let trades = parser::Trade::parse_many(&link).await.unwrap();
    let mut links: Vec<String> = vec![];

    trades.iter().for_each(|trade| {
        let mut has_cells: Vec<String> = trade
            .has
            .iter()
            .filter(|cell| !IGNORE.contains(&cell.item.as_str())).map(|cell| {
                format!("https://rocket-league.com/trading?filterItem={}&filterCertification={}&filterPaint={}&filterSeries={}&filterRarity={}&filterTradeType=2&filterSearchType=1&filterItemType={}&filterPlatform[]={}", cell.item, cell.certification, cell.paint, cell.series, cell.quality,cell.item_type, trade.platform)
            }).collect();

        let mut wants_cells: Vec<String> = trade
            .wants
            .iter()
            .filter(|cell| !IGNORE.contains(&cell.item.as_str())).map(|cell| {
                format!("https://rocket-league.com/trading?filterItem={}&filterCertification={}&filterPaint={}&filterSeries={}&filterRarity={}&filterTradeType=2&filterSearchType=2&filterItemType={}&filterPlatform[]={}", cell.item, cell.certification, cell.paint, cell.series, cell.quality,cell.item_type, trade.platform)
            }).collect();

        has_cells.append(&mut wants_cells);

        links.append(&mut has_cells);
    });

    links
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
    let assets = assets::assets::Assets::new(Some(false)).await.unwrap();

    tauri::Builder::default()
        .manage(AssetsState(Mutex::new(assets)))
        .invoke_handler(tauri::generate_handler![
            links_to_cells,
            parse,
            parse_trade_links,
            parse_profile_links,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
