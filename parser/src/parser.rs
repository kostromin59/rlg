use std::{borrow::Cow, error::Error};
use url;

use scraper::{
    element_ref::Select, selector::CssLocalName, CaseSensitivity, Element, ElementRef, Html,
    Selector,
};

#[derive(Debug)]
pub struct Cell {
    pub item: String,
    pub certification: String,
    pub item_type: String,
    pub quality: String,
    pub series: String,
    pub paint: String,
    pub count: usize,
}

const BASE_URL: &'static str = "https://rocket-league.com/";

impl Cell {
    pub fn from_field(field: &Select) -> Result<Vec<Self>, Box<dyn Error>> {
        let cell_selector = Selector::parse(".rlg-item")?;
        let field = field.clone().next().unwrap();
        let cells = field.select(&cell_selector);

        let mut parsed: Vec<Self> = vec![];

        for cell in cells {
            let links_selector = Selector::parse(".rlg-item-links")?;
            let mut links = cell.select(&links_selector);

            let link_selector = Selector::parse(".rlg-btn-secondary")?;
            let link = links.next().unwrap().select(&link_selector).next().unwrap();

            let params = link.value().attr("href").unwrap();
            let base_url = url::Url::parse(BASE_URL)?;
            let url_with_params = base_url.join(params)?;

            let params = url_with_params.query_pairs();

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

            let count_selector = Selector::parse(".rlg-item__quantity")?;
            let mut count_element = cell.select(&count_selector);

            let count: usize = match count_element.next() {
                Some(element) => element.inner_html().trim().parse::<usize>().unwrap_or(1),
                None => 1,
            };

            parsed.push(Self {
                item,
                certification,
                item_type,
                quality,
                series,
                paint,
                count,
            })
        }

        Ok(parsed)
    }
}

#[derive(Debug)]
pub struct Trade {
    pub id: String,
    pub has: Vec<Cell>,
    pub wants: Vec<Cell>,
    pub username: String,
    pub platform: String,
}

impl Trade {
    pub async fn parse_one(link: &str) -> Result<Self, Box<dyn Error>> {
        let document = Self::get_page(link).await?;
        let trade_selector = Selector::parse(".rlg-trade")?;
        let mut trades = document.select(&trade_selector);

        let trade = trades.next().unwrap();

        let parsed = Self::parse(&trade)?;

        Ok(parsed)
    }

    pub async fn parse_many(link: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        let document = Self::get_page(link).await?;
        let trade_selector = Selector::parse(".rlg-trade")?;
        let trades = document.select(&trade_selector);

        let mut parsed: Vec<Self> = vec![];

        for trade in trades {
            parsed.push(Self::parse(&trade)?);
        }

        Ok(parsed)
    }

    async fn get_page(link: &str) -> Result<Html, Box<dyn Error>> {
        let res = reqwest::get(link).await?.text().await?;
        Ok(Html::parse_document(&res))
    }

    fn parse(trade: &ElementRef) -> Result<Self, Box<dyn Error>> {
        let has_selector = Selector::parse(".rlg-trade__itemshas")?;
        let wants_selector = Selector::parse(".rlg-trade__itemswants")?;
        let has = trade.select(&has_selector);
        let wants = trade.select(&wants_selector);

        let has_cells = Cell::from_field(&has)?;
        let wants_cells = Cell::from_field(&wants)?;

        let id = trade.value().attr("data-trade").unwrap();

        let username_selector = Selector::parse(".rlg-trade__username")?;
        let username_element = trade.select(&username_selector).next().unwrap();
        let username = username_element.text().last().unwrap().trim();

        let platform_selector = Selector::parse(".rlg-trade__platform")?;

        let mut platform_container = trade.select(&platform_selector);

        let platform_container = loop {
            let container = platform_container.next().unwrap();
            if !container.has_class(
                &CssLocalName::from("--other"),
                CaseSensitivity::CaseSensitive,
            ) {
                break container;
            }
        };

        let platform_image_selector = Selector::parse(".rlg-trade__platformlogo")?;
        let platform_image = platform_container
            .select(&platform_image_selector)
            .next()
            .unwrap();

        let alt = platform_image.value().attr("alt").unwrap();

        let platform: String = match alt {
            "Windows PC" => "1".to_owned(),
            "PlayStation" => "2".to_owned(),
            "Xbox" => "3".to_owned(),
            "Nintendo Switch" => "4".to_owned(),
            _ => "0".to_owned(),
        };

        Ok(Self {
            id: id.to_owned(),
            has: has_cells,
            wants: wants_cells,
            username: username.to_owned(),
            platform,
        })
    }
}
