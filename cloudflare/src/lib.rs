use std::collections::HashMap;

use scraper::{Html, Selector};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct CloudflareResponse {
    response: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Cloudflare {
    solution: CloudflareResponse,
}

pub async fn get_page(link: String) -> Option<Html> {
    if let Some(doc) = scrape(&link).await {
        return Some(doc);
    }
    scrape_server(&link).await
}

async fn scrape_server(link: &str) -> Option<Html> {
    let mut body = HashMap::new();
    body.insert("cmd".to_string(), "request.get".to_string());
    body.insert("url".to_string(), link.to_string());

    let client = reqwest::Client::new();
    let res = client
        .post("http://ngsquad.ru:8191/v1")
        .json(&body)
        .send()
        .await
        .unwrap();

    let res: Cloudflare = serde_json::from_str(res.text().await.unwrap().as_ref()).unwrap();

    Some(Html::parse_document(&res.solution.response))
}

async fn scrape(link: &str) -> Option<Html> {
    let html = reqwest::get(link).await.unwrap().text().await.unwrap();
    let document = Html::parse_document(&html);

    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .unwrap()
        .inner_html();

    if title.contains("Rocket League Garage") {
        return Some(document);
    }

    None
}
