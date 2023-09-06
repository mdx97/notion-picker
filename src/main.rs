mod collector;

use std::env;
use std::process;

use notion::models::properties::PropertyValue;
use notion::NotionApi;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::collector::{Collector, GetAllDatabaseEntriesCollector, GetDatabaseByNameCollector};

/// Values of the "Status" columns in Notion databases that can be picked.
pub const WHITELIST_STATUSES: &[&str] = &[
    "listening",
    "partially watched",
    "paused",
    "playing",
    "reading",
    "shelved",
    "want to play",
    "want to read",
    "want to rewatch",
    "want to watch",
    "want to watch again",
    "watching",
];

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: notion-picker <database>");
        process::exit(1);
    }

    let dbname = &args[1];
    let token = env::var("NOTION_SECRET").unwrap();
    let notion = NotionApi::new(token).unwrap();
    let database = GetDatabaseByNameCollector::new(&notion, dbname)
        .collect()
        .await
        .unwrap()
        .unwrap();

    let pages = GetAllDatabaseEntriesCollector::new(&notion, database.id)
        .collect()
        .await
        .unwrap();

    let options: Vec<_> = pages
        .into_iter()
        .filter_map(|page| {
            let Some(title) = page.title() else {
                return None;
            };
            page.properties
                .properties
                .get("Status")
                .and_then(|value| {
                    if let PropertyValue::Status { status, .. } = value {
                        status.to_owned()
                    } else {
                        None
                    }
                })
                .and_then(|status| status.name)
                .map(|status| status.to_lowercase())
                .map(|status| WHITELIST_STATUSES.contains(&status.as_str()))
                .unwrap_or(false)
                .then_some(title)
        })
        .collect();

    let item = options.choose(&mut thread_rng()).unwrap();
    println!("{}", item);
}
