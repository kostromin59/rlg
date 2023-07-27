use parser::parser;

#[tokio::main]
async fn main() {
    let f = parser::Trade::parse_many("https://rocket-league.com/player/XallenzzRL").await;

    match f {
        Ok(trade) => println!("Done! {:#?} \n {}", trade, trade.len()),
        Err(err) => eprintln!("err {}", err),
    }
}
