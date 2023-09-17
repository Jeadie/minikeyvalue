mod rebalance;
mod s3api;
mod server;
mod app;
mod lib;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mkv", about = "Distributed key-value storage system.")]
enum Command {
    #[structopt(about = "Starts the master server.")]
    Server(app::App),

    #[structopt(about = "Rebalances the storage system.")]
    Rebalance {
        #[structopt(short, long)]
        db: String,

        #[structopt(short, long)]
        volumes: String,
    },

    #[structopt(about = "Rebuilds the LevelDB.")]
    Rebuild {
        #[structopt(short, long)]
        db: String,

        #[structopt(short, long)]
        volumes: String,
    },
}

fn main() {
    let command = Command::from_args();
    match command {
        Command::Server(_app) => {
            // app.run_server();
        }
        Command::Rebalance { db: _, volumes: _ } => {
            // 
        }
        Command::Rebuild { db: _, volumes: _ } => {
            // Rebuild logic here
        }
    }
}