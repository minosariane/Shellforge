use base64::Engine;
use inline_colorization::*;
use include_dir::{include_dir, Dir};

static FORMATS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/formats");

fn process_template(entry: &str, template: &str, ip: &str, port: u16, b64: bool, url: bool) {
    let coloured_ip = format!("{color_cyan}{}{color_reset}", ip);
    let coloured_port = format!("{color_cyan}{}{color_reset}", &port.to_string());
    let result = template
        .replace("{{IP}}", &coloured_ip)
        .replace("{{PORT}}", &coloured_port);

    println!("----------{}-----------\n{}", entry.to_ascii_uppercase(), result);

    if b64 {
        println!(
            "base64: {}",
            base64::prelude::BASE64_STANDARD.encode(&result)
        );
    }

    if url {
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

    if format == "all" {

        for file in FORMATS_DIR.files() {
            let path = file.path();

            let file_name = match path.file_stem().and_then(|s| s.to_str()) {
                Some(name) => name,
                None => {
                    eprintln!("Nom de fichier invalide: {:?}", path);
                    continue;
                }
            };

            let template = match file.contents_utf8() {
                Some(t) => t.to_string(),
                None => {
                    eprintln!("Erreur lecture fichier {:?}", path);
                    continue;
                }
            };

            process_template(file_name, &template, &ip, port, b64, url);
        }

    } else if format == "top" {
        let formats = [
            "bash-i", "nc-e", "python", "python3",
            "php", "socat", "perl", "ruby",
            "bash196", "telnet",
        ];

        for fmt in formats {
            let template = match get_template(fmt) {
                Some(t) => t,
                None => continue,
            };

            process_template(fmt, &template, &ip, port, b64, url);
        }

    } else {
        let template = match get_template(&format) {
            Some(t) => t,
            None => return,
        };

        process_template(&format, &template, &ip, port, b64, url);
    }
}

fn get_template(format: &str) -> Option<String> {
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
