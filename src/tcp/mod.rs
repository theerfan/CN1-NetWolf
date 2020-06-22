use crate::udp::{generate_address, get::GETPair};
use crate::{BUF_SIZE, STATIC_DIR};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};

const LOCALHOST: &str = "127.0.0.1";

fn generate_file_address(file_name: &str) -> String {
    let mut file_addr = String::from(STATIC_DIR);
    file_addr.push_str(file_name);
    file_addr
}

pub async fn tcp_get_receiver((ip, res): (String, GETPair)) -> std::io::Result<()> {
    let stream = TcpStream::connect(generate_address(&ip, res.tcp_port))?;
    let mut tcp_input_stream = BufReader::new(stream);
    let file_addr = generate_file_address(&res.file_name);
    let f = File::open(file_addr)?;
    let mut file_output_stream = BufWriter::new(f);
    let mut buf = [0; BUF_SIZE];
    let mut size: usize = 1;
    while size > 0 {
        size = tcp_input_stream.read(&mut buf)?;
        file_output_stream.write(&buf)?;
    }
    Ok(())
}

pub fn handle_client(stream: TcpStream, file_name: &str) -> std::io::Result<()> {
    let mut tcp_output_steam = BufWriter::new(stream);
    let file_addr = generate_file_address(file_name);
    let f = File::open(file_addr)?;
    let mut file_input_stream = BufReader::new(f);
    let mut buf = [0; BUF_SIZE];
    let mut size: usize = 1;
    while size > 0 {
        size = file_input_stream.read(&mut buf)?;
        tcp_output_steam.write(&buf)?;
    }
    Ok(())
}

pub async fn tcp_get_sender(starting_point: GETPair) -> std::io::Result<()> {
    let tcp_addr = generate_address(LOCALHOST, starting_point.tcp_port);
    let listener = match TcpListener::bind(tcp_addr) {
        Ok(lsner) => lsner,
        Err(_) => return Ok(()),
    };

    for stream in listener.incoming() {
        handle_client(stream?, &starting_point.file_name)?;
    }
    Ok(())
}
