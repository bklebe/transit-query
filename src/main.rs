use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use gtfs_schedule::GtfsSchedule;
use maplit::btreemap;
use serde::de::DeserializeOwned;
use trustfall::{execute_query, TransparentValue};

use crate::adapter::Adapter;

mod adapter;
mod gtfs_realtime;
mod gtfs_schedule;
pub use gtfs_realtime::*;

use clap::Parser;

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(default_value = "./query.gql")]
    query_file: PathBuf,
}

fn main() {
    let args = CliArgs::parse();

    let file_contents = std::fs::read_to_string(&args.query_file).expect(&format!(
        "Could not open file {}",
        args.query_file.display()
    ));

    let contents = get_feed("https://cdn.mbta.com/realtime/VehiclePositions.json");
    let trip_updates = get_feed("https://cdn.mbta.com/realtime/TripUpdates.json");
    let schedule = GtfsSchedule::from_path(Path::new("../MBTA_GTFS"));
    let adapter: Adapter = Adapter::new(&contents, &trip_updates, &schedule);
    let variables: BTreeMap<Arc<str>, Arc<str>> = BTreeMap::new(); // btreemap! {Arc::from("minLabel") => Arc::from("3900")};
    execute_query(Adapter::schema(), adapter.into(), &file_contents, variables)
        .expect("query failed to parse")
        .map(|v| {
            v.into_iter()
                .map(|(k, v)| (k, TransparentValue::from(v)))
                .collect::<BTreeMap<_, _>>()
        })
        .for_each(|result| {
            println!(
                "{}",
                serde_json::to_string_pretty(&result).expect("failed to serialize result")
            )
        });
}

fn get_feed<T>(url: &str) -> T
where
    T: DeserializeOwned,
{
    let body = reqwest::blocking::get(url)
        .unwrap_or_else(|_| panic!("couldn't pull {}", url))
        .text()
        .expect("invalid response encoding");
    deserialize_feed(&body)
}

pub(crate) fn deserialize_feed<T>(text: &str) -> T
where
    T: DeserializeOwned,
{
    serde_json::from_str(text).expect("couldn't deserialize")
}
