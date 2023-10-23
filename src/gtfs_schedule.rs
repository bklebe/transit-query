use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) struct GtfsSchedule {
    pub routes: Vec<Route>,
}

impl GtfsSchedule {
    pub(crate) fn from_path(path: &Path) -> Self {
        let routes_text =
            fs::read_to_string(path.join("routes.txt")).expect("couldn't read routes.txt");
        let routes = csv::Reader::from_reader(routes_text.as_bytes())
            .deserialize()
            .collect::<Result<Vec<Route>, _>>()
            .expect("couldn't deserialize routes.txt");

        Self { routes }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Route {
    pub(super) route_id: String,
    pub(super) route_long_name: String,
    pub(super) route_short_name: String,
}
