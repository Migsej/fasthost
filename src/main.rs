use std::io::prelude::*;
use std::process::Command;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use regex::Regex;
use clap::Parser;

mod arguments;
mod commands;

use arguments::ProgramArgs;

use commands::{curlify, wgetify};

fn main() {
    let args = ProgramArgs::parse();

    let ips = getips();

    let wgetcommands = ips.iter().map(|ip| wgetify(ip, args.port, &args.file));
    wgetcommands.for_each(|cmd| println!("{}", cmd));

    println!();

    let curlcommands = ips.iter().map(|ip| curlify(ip, args.port, &args.file));
    curlcommands.for_each(|cmd| println!("{}", cmd));


    let host: String = "0.0.0.0".to_string();
    let port = args.port.to_string();
    

    let contents = fs::read(args.file)
        .expect("Something went wrong reading the file");

    let end_point : String = host.to_owned() + ":" +  port.as_str();

    let listener = TcpListener::bind(end_point).unwrap();

    println!("Web server is listening at port {}",port);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        handle_connection(_stream, contents.clone());
    }
}

fn handle_connection(mut streem: TcpStream, mut contents: Vec<u8>) {
    let mut response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
            contents.len(),
    ).as_bytes().to_vec(); 
    response.append(&mut contents);
    streem.write(&response).unwrap();
    streem.flush().unwrap();

}

fn getips() -> Vec<String> {
    let filtered = vec!["127.0.0.1", "172.17.0.1"];
    
    let output = Command::new("ifconfig")
        .output()
        .expect("failed to execute `ifconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap();

    let re = Regex::new(r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#).unwrap();

    let mut result = Vec::new();

    for cap in re.captures_iter(&stdout) {
        if !filtered.contains(&&cap[2]) {
            result.push(cap[2].to_string());
        }
    }
    result

}

