use assets;
use parser;
use serde::{Deserialize, Serialize};
use std::{
    fs, process,
    thread::{self},
    time::Duration,
};
use teloxide::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    links: Vec<String>,
    token: String,
    user: String,
    time: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            links: vec![],
            token: String::new(),
            user: String::new(),
            time: 60 * 5,
        }
    }
}

struct Checker {
    pub trades: Vec<parser::parser::Trade>,
    pub links: Vec<String>,
    pub bot: Box<Bot>,
    pub user: String,
    pub assets: assets::assets::Assets,
}

impl Checker {
    fn new(
        links: &Vec<String>,
        bot: Box<Bot>,
        user: String,
        assets: assets::assets::Assets,
    ) -> Self {
        Self {
            trades: vec![],
            links: links.clone(),
            bot,
            user,
            assets,
        }
    }

    async fn update_trade(&mut self) {
        let mut parsed: Vec<parser::parser::Trade> = vec![];
        for link in &self.links {
            let trade = parser::parser::Trade::parse_one(&link).await;

            match trade {
                Ok(trade) => parsed.push(trade),
                Err(_) => continue,
            };
        }

        self.trades = parsed;
    }

    async fn compare_cells(
        &self,
        first: Option<&parser::parser::Cell>,
        second: Option<&parser::parser::Cell>,
        link: &str,
    ) {
        println!("{:?}\n{:?}", first, second);
        if first.is_none() && !second.is_none() {
            // Отправить в телеграм!
            // println!("Отличие: 1 элемент пустой, второй нет");
            let second = second.unwrap();

            let new_item = match self.assets.items.get(&second.item) {
                Some(item) => item.name.to_owned(),
                None => "Неизвестный предмет!".to_owned(),
            };

            let new_paint = self
                .assets
                .paints
                .get(&second.paint)
                .unwrap_or(&"Неизвестный цвет".to_owned())
                .to_owned();

            let new_quality = self
                .assets
                .qualities
                .get(&second.quality)
                .unwrap_or(&"Неизвестная редкость!".to_owned())
                .to_owned();

            let new_cert = self
                .assets
                .certifications
                .get(&second.certification)
                .unwrap_or(&"Неизвестная сертификация!".to_owned())
                .to_owned();

            let new_series = self
                .assets
                .series
                .get(&second.series)
                .unwrap_or(&"Неизвестная серия!".to_owned())
                .to_owned();

            let new_type = self
                .assets
                .item_types
                .get(&second.item_type)
                .unwrap_or(&"Неизвестный тип предмета!".to_owned())
                .to_owned();

            let second_count = second.count;
            let _ = self.bot.send_message(
                self.user.clone(),
                format!("Отличие!\n\nБыло: ничего\n\nСтало:\nНазвание: {new_item}\nЦвет: {new_paint}\nКачество: {new_quality}\nСертификация: {new_cert}\nСерия: {new_series}\nТип предмета: {new_type}\nКоличество: {second_count}\n\n{link}"),
            ).await;
            return;
        } else if !first.is_none() && second.is_none() {
            // Отправить в телеграм!
            // println!("Отличие: 2 элемент пустой, певый нет");
            let first = first.unwrap();
            let first_count = first.count;
            let was_item = match self.assets.items.get(&first.item) {
                Some(item) => item.name.to_owned(),
                None => "Неизвестный предмет!".to_owned(),
            };

            let was_paint = self
                .assets
                .paints
                .get(&first.paint)
                .unwrap_or(&"Неизвестный цвет".to_owned())
                .to_owned();

            let was_quality = self
                .assets
                .qualities
                .get(&first.quality)
                .unwrap_or(&"Неизвестная редкость!".to_owned())
                .to_owned();

            let was_cert = self
                .assets
                .certifications
                .get(&first.certification)
                .unwrap_or(&"Неизвестная сертификация!".to_owned())
                .to_owned();

            let was_series = self
                .assets
                .series
                .get(&first.series)
                .unwrap_or(&"Неизвестная серия!".to_owned())
                .to_owned();

            let was_type = self
                .assets
                .item_types
                .get(&first.item_type)
                .unwrap_or(&"Неизвестный тип предмета!".to_owned())
                .to_owned();

            let _ = self.bot.send_message(
                self.user.clone(),
                format!("Отличие!\n\nБыло:\nНазвание: {was_item}\nЦвет: {was_paint}\nКачество: {was_quality}\nСертификация: {was_cert}\nСерия: {was_series}\nТип предмета: {was_type}\nКоличество: {first_count}\n\nСтало: ничего\n\n{link}"),
            ).await;
            return;
        }

        // Ячейки не пустые
        let first = first.unwrap();
        let second = second.unwrap();

        if first.item != second.item
            || first.paint != second.paint
            || first.count != second.count
            || first.series != second.series
            || first.quality != second.quality
            || first.item_type != second.item_type
            || first.certification != second.certification
        {
            // Отправить в телеграм!
            println!("Отличие!");

            let was_item = match self.assets.items.get(&first.item) {
                Some(item) => item.name.to_owned(),
                None => "Неизвестный предмет!".to_owned(),
            };

            let was_paint = self
                .assets
                .paints
                .get(&first.paint)
                .unwrap_or(&"Неизвестный цвет".to_owned())
                .to_owned();

            let was_quality = self
                .assets
                .qualities
                .get(&first.quality)
                .unwrap_or(&"Неизвестная редкость!".to_owned())
                .to_owned();

            let was_cert = self
                .assets
                .certifications
                .get(&first.certification)
                .unwrap_or(&"Неизвестная сертификация!".to_owned())
                .to_owned();

            let was_series = self
                .assets
                .series
                .get(&first.series)
                .unwrap_or(&"Неизвестная серия!".to_owned())
                .to_owned();

            let was_type = self
                .assets
                .item_types
                .get(&first.item_type)
                .unwrap_or(&"Неизвестный тип предмета!".to_owned())
                .to_owned();

            let new_item = match self.assets.items.get(&second.item) {
                Some(item) => item.name.to_owned(),
                None => "Неизвестный предмет!".to_owned(),
            };

            let new_paint = self
                .assets
                .paints
                .get(&second.paint)
                .unwrap_or(&"Неизвестный цвет".to_owned())
                .to_owned();

            let new_quality = self
                .assets
                .qualities
                .get(&second.quality)
                .unwrap_or(&"Неизвестная редкость!".to_owned())
                .to_owned();

            let new_cert = self
                .assets
                .certifications
                .get(&second.certification)
                .unwrap_or(&"Неизвестная сертификация!".to_owned())
                .to_owned();

            let new_series = self
                .assets
                .series
                .get(&second.series)
                .unwrap_or(&"Неизвестная серия!".to_owned())
                .to_owned();

            let new_type = self
                .assets
                .item_types
                .get(&second.item_type)
                .unwrap_or(&"Неизвестный тип предмета!".to_owned())
                .to_owned();

            let second_count = second.count;
            let first_count = first.count;

            let _ = self.bot.send_message(
                self.user.clone(),
                format!("Отличие!\n\nБыло:\nНазвание: {was_item}\nЦвет: {was_paint}\nКачество: {was_quality}\nСертификация: {was_cert}\nСерия: {was_series}\nТип предмета: {was_type}\nКоличество: {first_count}\n\nСтало:\nНазвание: {new_item}\nЦвет: {new_paint}\nКачество: {new_quality}\nСертификация: {new_cert}\nСерия: {new_series}\nТип предмета: {new_type}\nКоличество: {second_count}\n\n{link}"),
            ).await;
        }
    }

    async fn compare(&mut self, link: &str) {
        let trade = parser::parser::Trade::parse_one(link).await;
        match trade {
            Ok(trade) => {
                let saved_trade = self.trades.iter().find(|saved| saved.id == trade.id);

                if saved_trade.is_none() {
                    return println!(
                        "Такого трейда не найдено!\nhttps://rocket-league.com/trade/{}",
                        trade.id
                    );
                }

                let saved_trade = saved_trade.unwrap();

                // Сравнить wants
                for i in 0..saved_trade.wants.len().max(trade.wants.len()) {
                    let first = saved_trade.wants.get(i);
                    let second = trade.wants.get(i);

                    self.compare_cells(first, second, link).await;
                }

                // Сравнить has
                for i in 0..saved_trade.has.len().max(trade.has.len()) {
                    let first = saved_trade.has.get(i);
                    let second = trade.has.get(i);

                    self.compare_cells(first, second, link).await;
                }
            }
            Err(_) => println!("Ошибка парсинга трейда {}", link),
        }
    }
}

#[tokio::main]
async fn main() {
    let config = fs::read_to_string("config.json").unwrap_or_else(|_| {
        eprintln!("Ошибка чтения конфига!");
        let default_config = Config::default();
        fs::write(
            "config.json",
            serde_json::to_string(&default_config).unwrap(),
        )
        .unwrap();
        println!("Конфиг создан, заполните его!");

        process::exit(1);
    });

    let config: Config = serde_json::from_str(&config).unwrap_or_else(|_| {
        eprintln!("Ошибка парсинга конфига!");
        process::exit(1);
    });

    let bot = Bot::new(config.token);

    println!("Конфиг успешно получен!");

    let assets = assets::assets::Assets::new().await.unwrap();
    let mut checker = Checker::new(&config.links, Box::new(bot), config.user, assets);

    println!("Получаю значения трейда!");
    checker.update_trade().await;

    loop {
        thread::sleep(Duration::from_secs(config.time));
        println!("Цикл");

        for link in &config.links {
            checker.compare(link).await;
        }

        checker.update_trade().await;
    }
}
