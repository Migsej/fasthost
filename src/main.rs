use std::io::prelude::*;
use std::process::Command;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use clap::Parser;
use regex::Regex;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to host
    #[arg(short, long)]
    file: String,

    /// port to host on
    #[arg(short, long, default_value = "12345")]
    port: u16,
}

fn main() {
    let args = Args::parse();

    let mut ips = getips();
    let wgetcommands = ips.iter().map(|ip| wgetify(ip, args.port, &args.file));
    wgetcommands.for_each(|cmd| println!("{}", cmd));

    let HOST: String = "0.0.0.0".to_string();
    let PORT = args.port.to_string();
    

    let mut contents = fs::read(args.file)
        .expect("Something went wrong reading the file");

    let end_point : String = HOST.to_owned() + ":" +  PORT.as_str();

    let listener = TcpListener::bind(end_point).unwrap();

    println!("Web server is listening at port {}",PORT);

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
    let output = Command::new("ifconfig")
        .output()
        .expect("failed to execute `ifconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap();

    let re = Regex::new(r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#).unwrap();

    let mut result = Vec::new();

    for cap in re.captures_iter(&stdout) {
        result.push(cap[2].to_string());
    }
    result

}

fn wgetify(ip: &String, port: u16, file: &String) -> String {
    let filename = file.split("/").last().unwrap();

    format!("wget http://{}:{} -O {}", ip, port, filename)
}

