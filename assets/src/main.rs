use std::process;

use assets;

#[tokio::main]
async fn main() {
    assets::parser::parse().await.unwrap_or_else(|err| {
        eprintln!("Произошла ошибка:\n{:?}", err);
        process::exit(1);
    });

    println!("Парсинг успешно завершён!");
}
