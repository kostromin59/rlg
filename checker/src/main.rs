use assets;
use parser;
use serde::{Deserialize, Serialize};
use std::{
    fs, process,
    thread::{self},
    time::Duration,
};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    links: Vec<String>,
}

struct Checker {
    pub trades: Vec<parser::parser::Trade>,
    pub links: Vec<String>,
}

impl Checker {
    fn new(links: &Vec<String>) -> Self {
        Self {
            trades: vec![],
            links: links.clone(),
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

    fn compare_cells(first: Option<&parser::parser::Cell>, second: Option<&parser::parser::Cell>) {
        println!("{:?}\n{:?}", first, second);
        if first.is_none() && !second.is_none() {
            // Отправить в телеграм!
            println!("Отличие: 1 элемент пустой, второй нет");
            return;
        } else if !first.is_none() && second.is_none() {
            // Отправить в телеграм!
            println!("Отличие: 2 элемент пустой, певый нет");
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
        }
    }

    async fn compare(&mut self, link: &str) {
        let trade = parser::parser::Trade::parse_one(link).await;
        match trade {
            Ok(trade) => {
                let saved_trade = self.trades.iter().find(|saved| saved.id == trade.id);

                if saved_trade.is_none() {
                    return println!("Такого трейда не найдено!");
                }

                let saved_trade = saved_trade.unwrap();

                // Сравнить wants
                for i in 0..saved_trade.wants.len().max(trade.wants.len()) {
                    let first = trade.wants.get(i);
                    let second = saved_trade.wants.get(i);

                    Self::compare_cells(first, second);
                }

                // Сравнить has
                for i in 0..saved_trade.has.len().max(trade.has.len()) {
                    let first = trade.has.get(i);
                    let second = saved_trade.has.get(i);

                    Self::compare_cells(first, second);
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
        process::exit(1);
    });

    let config: Config = serde_json::from_str(&config).unwrap_or_else(|_| {
        eprintln!("Ошибка парсинга конфига!");
        process::exit(1);
    });

    println!("Конфиг успешно получен!");

    let mut checker = Checker::new(&config.links);

    println!("Получаю значения трейда!");
    checker.update_trade().await;

    loop {
        thread::sleep(Duration::from_secs(20));
        println!("Цикл");

        for link in &config.links {
            checker.compare(link).await;
        }

        checker.update_trade().await;
    }
}
