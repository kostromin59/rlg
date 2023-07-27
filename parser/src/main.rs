use parser::parser;

#[tokio::main]
async fn main() {
    let f = parser::Trade::parse_one("https://rocket-league.com/trade/7gRRxo2").await;

    match f {
        Ok(trade) => println!("Done! {:#?}", trade),
        Err(err) => eprintln!("err {}", err),
    }

    println!("Hello, world!");
}
