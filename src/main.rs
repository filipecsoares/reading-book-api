mod api;
mod cli;
mod db;
mod model;

use std::env;

use api::*;
use model::params::Params;

fn main() {
    let params = Params::build(env::args()).unwrap();
    if params.service == "-api" {
        routes::run();
    } else {
        cli::cli::run();
    }
}
