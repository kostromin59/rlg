use std::error::Error;

use scraper::{ElementRef, Html, Selector};

pub struct Cell {
    pub id: String,
    pub item: String,
    pub certification: String,
    pub item_type: String,
    pub quality: String,
    pub series: String,
    pub paint: String,
    pub count: u8,
    // platform: String,
}

impl Cell {
    pub fn from_field(field: &ElementRef) -> Result<Vec<Self>, Box<dyn Error>> {
        Ok(vec![])
    }
}

pub struct Trade {
    pub id: String,
    pub has: Vec<Cell>,
    pub wants: Vec<Cell>,
    pub username: String,
    pub platform: String,
}

impl Trade {
    // https://rocket-league.com/trade/7gRRxo2
    pub async fn parse_one(link: &str) -> Result<Self, Box<dyn Error>> {
        let document = Self::get_page(link).await?;
        let trade_selector = Selector::parse(".rlg-trade")?;
        let mut trades = document.select(&trade_selector);

        let trade = trades.next().unwrap();

        let info = Self::parse(&trade)?;

        Ok(info)

        // let has_selector = Selector::parse(".rlg-trade__itemshas")?;
        // let wants_selector = Selector::parse("rlg-trade__itemswants")?;
        // let has = trade.select(&has_selector);
        // let wants = trade.select(&wants_selector);

        // Ok(Self {
        //     id: "".to_owned(),
        //     has: vec![],
        //     wants: vec![],
        //     username: "".to_owned(),
        //     platform: "".to_owned(),
        // })
    }
    pub fn parse_many(link: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        Ok(vec![])
    }

    async fn get_page(link: &str) -> Result<Html, Box<dyn Error>> {
        let res = reqwest::get(link).await?.text().await?;
        Ok(Html::parse_document(&res))
    }

    fn parse(trade: &ElementRef) -> Result<Self, Box<dyn Error>> {
        let has_selector = Selector::parse(".rlg-trade__itemshas")?;
        let wants_selector = Selector::parse("rlg-trade__itemswants")?;
        let has = trade.select(&has_selector);
        let wants = trade.select(&wants_selector);

        Ok(Self {
            id: "".to_owned(),
            has: vec![],
            wants: vec![],
            username: "".to_owned(),
            platform: "".to_owned(),
        })
    }

    // fn get_trades(document: &Html) -> Result<&Select, Box<dyn Error>> {
    // let trade_selector = Selector::parse(".rlg-trade")?;
    // let trade = document.select(&trade_selector);
    // Ok(&trade)
    // }
}
