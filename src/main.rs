extern crate plist;
use serde::{Serialize, Deserialize};
use rand::prelude::*;

enum Sections {
    PersistentApps,
    PersistentOthers,
    RecentApps,
    StaticApps,
    StaticOther,
}

enum Location {
    Begining,
    Middle,
    End,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Dockm {
    #[serde(rename = "static-only")]
    static_only: Option<bool>,
    #[serde(rename = "persistent-apps")]
    persistent_apps: Option<Vec<StaticItem>>,
    #[serde(rename = "persistent-others")]
    persistent_others: Option<Vec<StaticItem>>,
    #[serde(rename = "recent-apps")]
    recent_apps: Option<Vec<StaticItem>>,
    #[serde(rename = "static-apps")]
    static_apps: Option<Vec<StaticItem>>,
    #[serde(rename = "static-others")]
    static_others: Option<Vec<StaticItem>>,
}

impl Dockm {
    fn add_item_to_section(mut self, item: StaticItem, section: Sections) {
        match section {
            Sections::PersistentApps => self.persistent_apps.as_mut().map(|v| v.push(item)),
            Sections::PersistentOthers => self.persistent_others.as_mut().map(|v| v.push(item)),
            Sections::RecentApps => self.recent_apps.as_mut().map(|v| v.push(item)),
            Sections::StaticApps => self.static_apps.as_mut().map(|v| v.push(item)),
            Sections::StaticOther => self.static_others.as_mut().map(|v| v.push(item)),
        };
    }

    fn add_item_to_section_with_location(mut self, item: StaticItem, section: Sections, location: Location) {
        match section {
            Sections::PersistentApps => {
                match location {
                    Location::Begining => self.persistent_apps.as_mut().map(|v| v.push(item)),
                    Location::Middle => self.persistent_apps.as_mut().map(|v| v.push(item)),
                    Location::End => self.persistent_apps.as_mut().map(|v| v.push(item)),
                };
            },
            Sections::PersistentOthers => { 
                match location {
                    Location::Begining => self.persistent_apps.as_mut().map(|v| v.insert(0, item)),
                    Location::Middle => self.persistent_apps.as_mut().map(|v| v.insert(0, item)),
                    Location::End => self.persistent_apps.as_mut().map(|v| v.push(item)),
                };
             },
            Sections::RecentApps => { self.recent_apps.unwrap().push(item.to_owned()); },
            Sections::StaticApps => { self.static_apps.unwrap().push(item.to_owned()); },
            Sections::StaticOther => { self.static_others.unwrap().push(item.to_owned()); },
        }
    }
}

fn main() {
    // let dock = Value::from_file("src/com.apple.dock.plist").unwrap();
    // dock.to_file_xml("src/com.apple.dock.xml");
    let dock: Dockm = plist::from_file("src/com.apple.dock.plist").unwrap();

    // dock.add_item_to_section(StaticItem::new_file_tile("Google Chrome.app"), Sections::PersistentApps);
    dock.add_item_to_section_with_location(StaticItem::new_file_tile("Google Chrome.app"), Sections::PersistentApps, Location::Begining);

    let persistent_apps = dock.persistent_apps.unwrap();

    println!("{0: <45} | {1: <140} | {2: <10} | {3: <10} | {4: <10}", "Label", "FilePath", "FileType", "ShowAs", "Arrangement");
    for item in persistent_apps {
        println!("{0: <45} | {1: <140} | {2: <10} | {3: <10} | {4: <10}", item.tile_data.file_label, item.tile_data.file_data.cf_url_string, item.tile_data.file_data.cf_url_string_type, item.tile_data.display_as.unwrap_or_default(), item.tile_data.arrangement.unwrap_or_default())
    }

    // println!("{:#?}", StaticItem::new_file_tile("Google Chrome.app"));

    // dock.add_item()

    // let tile = dock.unwrap().as_dictionary().unwrap()["persistent-others"].;
    // println!("{:#?}", dock.unwrap().as_dictionary().unwrap()["persistent-others"]);
    // println!("{:#?}", dock.recent_apps.unwrap());
    // dock.add_item("/test/dsg", "recent-apps", "first").expect("Adding new item")

    // for tile in dock {
    //     println!("{:#?}", tile);
    // }
}
