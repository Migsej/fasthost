pub fn wgetify(ip: &String, port: u16, file: &String) -> String {
    let filename = file.split("/").last().unwrap();

    format!("wget http://{}:{} -O {}", ip, port, filename)
}

pub fn curlify(ip: &String, port: u16, file: &String) -> String {
    let filename = file.split("/").last().unwrap();

    format!("curl http://{}:{} -o {}", ip, port, filename)
}
