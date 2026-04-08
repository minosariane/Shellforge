use clap::{Parser, Subcommand};
use include_dir::{Dir, include_dir};

static FORMATS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/formats");

mod commands;

#[derive(Parser)]
#[command(name = "shellforge")]
#[command(about = "Reverse shell generator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short, long)]
        ip: String,

        #[arg(short, long, default_value = "7777")]
        port: u16,

        #[arg(short, long, default_value = "top")]
        format: String,

        #[arg(short, long, help = "Encode to base64")]
        b64: bool,

        #[arg(short, long, help = "Encode to Url")]
        url: bool,
    },

    ListFormats,
}

fn main() {
    println!(
        r"
 _______           _______  _        _        ______________    _______  _______  _______  _______  _______ 
(  ____ \|\     /|(  ____ \( \      ( \      ( |          | )  (  ____ \(  ___  )(  ____ )(  ____ \(  ____ \
| (    \/| )   ( || (    \/| (      | (      | |          | |  | (    \/| (   ) || (    )|| (    \/| (    \/
| (_____ | (___) || (__    | |      | |      (_|__________|_)  | (__    | |   | || (____)|| |      | (__    
(_____  )|  ___  ||  __)   | |      | |            |  |        |  __)   | |   | ||     __)| | ____ |  __)   
      ) || (   ) || (      | |      | |            |  |        | (      | |   | || (\ (   | | \_  )| (      
/\____) || )   ( || (____/\| (____/\| (____/\      |  |        | )      | (___) || ) \ \__| (___) || (____/\
\_______)|/     \|(_______/(_______/(_______/      |  |        |/       (_______)|/   \__/(_______)(_______/
                                                   |__|                                        version 1.0.1
                                                   (__)"
    );

    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            ip,
            port,
            format,
            b64,
            url,
        } => {
            commands::generate::run(ip, port, format, b64, url);
        }

        Commands::ListFormats => {
            commands::list::run();
        }
    }
}
