use crate::assets::{self, Item};
use scraper::{element_ref, html::Select, Html, Selector};
use std::{collections::HashMap, error::Error, fs, path::Path};

const LINK: &str = "https://rocket-league.com/trades/Botlox";

pub async fn parse(save: Option<bool>) -> Result<assets::Assets, Box<dyn Error>> {
    let res = reqwest::get(LINK).await?.text().await?;
    let document = Html::parse_document(&res);

    let items_selector = Selector::parse("select#filterItem")?;
    let items = parse_items(document.select(&items_selector));

    let certifications_selector = Selector::parse("select#filterCertification")?;
    let paints_selector = Selector::parse("select#filterPaint")?;
    let series_selector = Selector::parse("select#filterSeries")?;
    let qualities_selector = Selector::parse("select#filterRarity")?;
    let platforms_selector = Selector::parse("select#filterPlatform")?;
    let search_types_selector = Selector::parse("select#filterSearchType")?;
    let item_type_selector = Selector::parse("select#filterItemType")?;

    let certifications = parse_select(document.select(&certifications_selector));
    let paints = parse_select(document.select(&paints_selector));
    let series = parse_select(document.select(&series_selector));
    let qualities = parse_select(document.select(&qualities_selector));
    let platforms = parse_select(document.select(&platforms_selector));
    let search_types = parse_select(document.select(&search_types_selector));
    let item_types = parse_select(document.select(&item_type_selector));

    let save = save.unwrap_or(false);

    if save {
    save_json(serde_json::to_string(&items)?, "items.json")?;
        save_json(
            serde_json::to_string(&certifications)?,
            "certifications.json",
        )?;
        save_json(serde_json::to_string(&paints)?, "paints.json")?;
        save_json(serde_json::to_string(&series)?, "series.json")?;
        save_json(serde_json::to_string(&qualities)?, "qualities.json")?;
        save_json(serde_json::to_string(&platforms)?, "platforms.json")?;
        save_json(serde_json::to_string(&search_types)?, "search_types.json")?;
        save_json(serde_json::to_string(&item_types)?, "item_types.json")?;
    }

    Ok(assets::Assets {
        items: HashMap::new(),
        certifications,
        paints,
        series,
        qualities,
        platforms,
        search_types,
        item_types,
    })
}

fn save_json(asset: String, name: &str) -> Result<(), Box<dyn Error>> {
    let is_dir_exists = Path::new("parsed").exists();

    if !is_dir_exists {
        fs::create_dir("parsed")?;
    }

    let path = Path::new("parsed").join(name);

    fs::write(path, asset)?;

    Ok(())
}

fn parse_select(selector: Select) -> assets::Asset {
    let selector = selector.into_iter().next().unwrap();
    let options_selector = Selector::parse("option").unwrap();

    let mut parsed: assets::Asset = HashMap::new();

    for option in selector.select(&options_selector) {
        let key = option.value().attr("value").unwrap().trim();
        let value = option.text().next();

        if value.is_none() {
            continue;
        }

        parsed.insert(key.to_owned(), value.unwrap().trim().to_owned());
    }

    parsed
}

fn parse_items(selector: Select) -> HashMap<String, Item> {
    let selector = selector.into_iter().next().unwrap();
    let miscellaneous_selector = Selector::parse("optgroup[label=\"Miscellaneous\"]").unwrap();
    let bodies_selector = Selector::parse("optgroup[label=\"Bodies\"]").unwrap();
    let wheels_selector = Selector::parse("optgroup[label=\"Wheels\"]").unwrap();
    let decals_selector = Selector::parse("optgroup[label=\"Decals\"]").unwrap();
    let rocket_boosts_selector = Selector::parse("optgroup[label=\"Rocket Boosts\"]").unwrap();
    let goal_explosions_selector = Selector::parse("optgroup[label=\"Goal Explosions\"]").unwrap();
    let toppers_selector = Selector::parse("optgroup[label=\"Toppers\"]").unwrap();
    let paint_finishes_selector = Selector::parse("optgroup[label=\"Paint Finishes\"]").unwrap();
    let antennas_selector = Selector::parse("optgroup[label=\"Antennas\"]").unwrap();
    let trails_selector = Selector::parse("optgroup[label=\"Trails\"]").unwrap();
    let banners_selector = Selector::parse("optgroup[label=\"Banners\"]").unwrap();
    let engine_audio_selector = Selector::parse("optgroup[label=\"Engine Audio\"]").unwrap();
    let avatar_borders_selector = Selector::parse("optgroup[label=\"Avatar Borders\"]").unwrap();

    let mut parsed: HashMap<String, Item> = HashMap::new();

    let miscellaneous = selector.select(&miscellaneous_selector);
    let bodies = selector.select(&bodies_selector);
    let wheels = selector.select(&wheels_selector);
    let decals = selector.select(&decals_selector);
    let rocket_boosts = selector.select(&rocket_boosts_selector);
    let goal_explosions = selector.select(&goal_explosions_selector);
    let toppers = selector.select(&toppers_selector);
    let paint_finishes = selector.select(&paint_finishes_selector);
    let antennas = selector.select(&antennas_selector);
    let trails = selector.select(&trails_selector);
    let banners = selector.select(&banners_selector);
    let engine_audio = selector.select(&engine_audio_selector);
    let avatar_borders = selector.select(&avatar_borders_selector);

    parsed.extend(parse_items_categories(
        miscellaneous,
        assets::ItemCategories::Miscellaneous,
    ));

    parsed.extend(parse_items_categories(
        bodies,
        assets::ItemCategories::Bodies,
    ));

    parsed.extend(parse_items_categories(
        wheels,
        assets::ItemCategories::Wheels,
    ));

    parsed.extend(parse_items_categories(
        decals,
        assets::ItemCategories::Decals,
    ));

    parsed.extend(parse_items_categories(
        rocket_boosts,
        assets::ItemCategories::RocketBoosts,
    ));

    parsed.extend(parse_items_categories(
        goal_explosions,
        assets::ItemCategories::GoalExplosions,
    ));

    parsed.extend(parse_items_categories(
        toppers,
        assets::ItemCategories::Toppers,
    ));

    parsed.extend(parse_items_categories(
        paint_finishes,
        assets::ItemCategories::PaintFinishes,
    ));

    parsed.extend(parse_items_categories(
        antennas,
        assets::ItemCategories::Antennas,
    ));

    parsed.extend(parse_items_categories(
        trails,
        assets::ItemCategories::Trails,
    ));

    parsed.extend(parse_items_categories(
        banners,
        assets::ItemCategories::Banners,
    ));

    parsed.extend(parse_items_categories(
        engine_audio,
        assets::ItemCategories::EngineAudio,
    ));

    parsed.extend(parse_items_categories(
        avatar_borders,
        assets::ItemCategories::AvatarBorders,
    ));

    parsed
}

fn parse_items_categories(
    selector: element_ref::Select,
    category: assets::ItemCategories,
) -> HashMap<String, Item> {
    let selector = selector.into_iter().next().unwrap();
    let options_selector = Selector::parse("option").unwrap();

    let mut parsed: HashMap<String, Item> = HashMap::new();

    for option in selector.select(&options_selector) {
        let key = option.value().attr("value").unwrap();
        let name = option.inner_html().trim().to_owned();

        let series_value = option.value().attr("data-series");
        let series = convert_string_array(series_value);

        let qualities_value = option.value().attr("data-rarities");
        let qualities = convert_string_array(qualities_value);

        parsed.insert(
            key.to_string(),
            Item {
                name,
                series,
                rarities: qualities,
                category: category.to_string(),
            },
        );
    }

    parsed
}

fn convert_string_array(value: Option<&str>) -> Vec<u8> {
    match value {
        Some(series) => {
            let splitted = series[1..series.len() - 1].split(", ");
            let converted = splitted.filter_map(|x| {
                let number = x.parse::<u8>();
                match number {
                    Ok(n) => Some(n),
                    Err(_) => None,
                }
            });

            converted.collect()
        }
        None => vec![],
    }
}
