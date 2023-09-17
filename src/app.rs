use leveldb::database::bytes;
use structopt::StructOpt;
use std::collections::HashSet;

use crate::{rebalance, lib::Record};

fn parse_hashset(src: &str) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    Ok(src.split(',').map(|s| s.to_string()).collect())
}


#[derive(StructOpt, Debug)]
pub struct App {
    /// Port for the server to listen on
    #[structopt(short = "p", long = "port", default_value = "3000")]
    port: u16,

    /// Path to database
    #[structopt(short = "d", long = "db")]
    db_path: String,

    /// Volumes to use for storage, comma separated
    #[structopt(long = "volumes")]
    pub volumes: Vec<String>,

    /// Fallback server for missing keys
    #[structopt(long = "fallback", default_value = "")]
    fallback: String,

    /// Amount of replicas to make of the data
    #[structopt(long = "replicas", default_value = "3")]
    pub replicas: usize,

    /// Amount of subvolumes, disks per machine
    #[structopt(long = "subvolumes", default_value = "10")]
    pub subvolumes: usize,

    /// Force UNLINK before DELETE
    #[structopt(long = "protect")]
    protect: bool,

    /// Calculate and store MD5 checksum of values
    #[structopt(long = "md5sum")] // , default_value = true)]
    md5sum: bool,

    /// Volume servers must respond in this amount of time or they are considered down (in seconds)
    #[structopt(long = "voltimeout", default_value = "1")]
    voltimeout: u64,

    // Non-CLI fields
    // db: Option<Database<&'a [u8]>>,

    // lock: std::sync::Mutex<i32>,

    #[structopt(parse(try_from_str = parse_hashset))]
    uploadids: std::collections::HashSet<String>,
}


impl App {
    fn new() -> Self {
        let mut app: App = App::from_args();
        // app.db = Some(Database::open(&app.db_path, Option::None));
        // app.lock = std::sync::Mutex::new(());
        app.uploadids = std::collections::HashSet::new();
        app
    }

    fn rebalance(&self) {
        rebalance::All(&self)
    }

    pub fn put_record(&self, key: Vec<u8>, rec: Record) -> bool {
        return true
    }
}
