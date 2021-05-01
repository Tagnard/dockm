extern crate plist;
use serde::{Serialize, Deserialize};
use rand::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StaticItem {
    #[serde(rename = "GUID")]
    guid: u32,
    #[serde(rename = "tile-type")]
    tile_type: String,
    #[serde(rename = "tile-data")]
    tile_data: TileData,
}

impl StaticItem {
    fn new(tile_data: TileData, file_type: &str) -> Self {
        let mut rng = rand::thread_rng();

        StaticItem {
            guid: rng.gen(),
            tile_type: file_type.to_owned(),
            tile_data: tile_data,
        }
    }

    fn new_file_tile(path: &str) -> Self {      
        let label_base = path.split("/").last().unwrap();
        let label: &str = &label_base[..label_base.len()-4];

        let tile_data = TileData {
            file_type: 32,
            file_data: FileData::new(path, 15).unwrap(),
            file_label: label.to_owned(),
            arrangement: None,
            directory: None,
            display_as: None,
        };
        StaticItem::new(tile_data, "file-tile")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TileData {
    #[serde(rename = "file-type")]
    file_type: u32,
    #[serde(rename = "file-data")]
    file_data: FileData,
    #[serde(rename = "file-label")]
    file_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory: Option<bool>,
    #[serde(rename = "displayas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_as: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arrangement: Option<u32>
}

impl TileData {
    fn new() -> Self {
        TileData {
            arrangement: None,
            display_as: None,
            directory: None,
            file_type: 0,
            file_data: FileData::new("", 0).unwrap(),
            file_label: "".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileData {
    #[serde(rename = "_CFURLStringType")]
    cf_url_string_type: u32,
    #[serde(rename = "_CFURLString")]
    cf_url_string: String
}

impl FileData {
    fn new(path: &str, r#type: u32) -> Result<Self, &str> {
        Ok(FileData {
            cf_url_string_type: r#type,
            cf_url_string: path.to_owned(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Dockm {
    #[serde(rename = "static-only")]
    static_only: Option<bool>,
    #[serde(rename = "persistent-apps")]
    persistent_apps: Vec<StaticItem>,
    #[serde(rename = "persistent-others")]
    persistent_others: Vec<StaticItem>,
    #[serde(rename = "recent-apps")]
    recent_apps: Vec<StaticItem>,
}

impl Dockm {
    fn add_item(&mut self, item: StaticItem) {
        self.persistent_apps.push(item);
    }
}
fn main() {
    println!("Hello, world!");
}
