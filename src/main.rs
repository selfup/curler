use std::io;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    make_html_file(split_it_up(make_a_request("URL_GOES_HERE"))).unwrap();
}

fn split_it_up(names: Vec<u8>) -> Vec<String> {
    let names_str = String::from_utf8_lossy(&names);
    let v: Vec<&str> = names_str.split('\n').collect();
    let mut new_vec = Vec::new();
    for i in v {
        new_vec.push(i.to_string());
    }
    new_vec
}

fn make_a_request(website: &str) -> Vec<u8> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("curl {}", website))
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    output.stdout
}

fn make_html_file(incoming_vec: Vec<String>) -> io::Result<()> {
    if Path::new("./websites/").exists() {
        return Ok(());
    }

    fs::create_dir("websites").unwrap();

    let all_markdown = Path::new("./websites/webpage.html");

    let mut buffer = try!(File::create(all_markdown));
    try!(buffer.write_all(incoming_vec.join("\n").as_bytes()));

    Ok(())
}
