use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mkv", about = "Distributed key-value storage system.")]
enum Command {
    #[structopt(about = "Starts the master server.")]
    Server {
        #[structopt(short, long, default_value = "/tmp/indexdb/")]
        db: String,

        #[structopt(short, long, default_value = "3000")]
        port: i32,

        #[structopt(short, long)]
        volumes: String,

        #[structopt(short, long, default_value = "3")]
        replicas: usize,

        #[structopt(short, long, default_value = "10")]
        subvolumes: usize,

        #[structopt(short, long)]
        fallback: Option<String>,

        #[structopt(long)]
        protect: bool,
    },

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
        Command::Server { db, port, volumes, replicas, subvolumes, fallback, protect } => {
            println!("Hello, server!");
        },
        Command::Rebalance { db, volumes } => {
            println!("Hello, rebalance!");
        },
        Command::Rebuild { db, volumes } => {
            println!("Hello, rebuild");
        },
    }
}
