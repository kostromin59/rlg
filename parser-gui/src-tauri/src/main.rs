// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use parser::parser::Cell;

#[tauri::command]
async fn links_to_cells(links: Vec<String>) -> Vec<Cell> {
    // Ошибка где-то в функции parse
    let assets = assets::assets::Assets::read_files().unwrap();

    let cells = links
        .iter()
        .map(|link| {
            let url = url::Url::parse(link).unwrap();
            Cell::from_link(&url)
        })
        .map(|cell| {
            let item = match assets.items.get(&cell.item) {
                Some(item) => item.name.to_owned(),
                None => "Неизвестный предмет!".to_owned(),
            };

            let paint = assets
                .paints
                .get(&cell.paint)
                .unwrap_or(&"Неизвестный цвет!".to_owned())
                .to_owned();

            let quality = assets
                .qualities
                .get(&cell.quality)
                .unwrap_or(&"Неизвестная редкость!".to_owned())
                .to_owned();

            let certification = assets
                .certifications
                .get(&cell.certification)
                .unwrap_or(&"Неизвестная сертификация!".to_owned())
                .to_owned();

            let series = assets
                .series
                .get(&cell.series)
                .unwrap_or(&"Неизвестная серия!".to_owned())
                .to_owned();

            let item_type = assets
                .item_types
                .get(&cell.item_type)
                .unwrap_or(&"Неизвестный тип предмета!".to_owned())
                .to_owned();

            Cell {
                item,
                item_type,
                series,
                certification,
                quality,
                paint,
                count: 0,
            }
        })
        .collect();

    cells
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![links_to_cells])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
