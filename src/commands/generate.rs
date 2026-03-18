use std::fs;
use base64::Engine;
use inline_colorization::*;

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
        let entries = match fs::read_dir("./formats") {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Erreur lecture dossier: {}", e);
                return;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();

            let file_name = match path.file_stem().and_then(|s| s.to_str()) {
                Some(name) => name,
                None => {
                    eprintln!("Nom de fichier invalide: {:?}", path);
                    continue;
                }
            };

            let template = match get_template(file_name) {
                Some(t) => t,
                None => continue,
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
    let path = format!("formats/{}.sf", format);

    match fs::read_to_string(&path) {
        Ok(t) => Some(t),
        Err(e) => {
            eprintln!("Erreur lecture fichier {}: {}", path, e);
            None
        }
    }
}