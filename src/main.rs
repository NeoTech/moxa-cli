//#[warn(unused_must_use)]

use clap::Parser;
use clap::CommandFactory;
use std::io::{self, Read, Write};
use std::net::{TcpStream};
use std::fs::File;
use std::thread;
use std::time::{Duration};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Path to the file to send
    #[clap(short = 'f', long)]
    file: Option<String>,
    // IP address of the device
    #[clap(short = 'n', long)]
    ip: Option<String>,
    // Port to use for TCP connection: Default is 4001
    #[clap(short = 'p', long, default_value = "4001")]
    port: u16,
    // Bytes to send per packet: Default is 1024
    #[clap(short = 'P', long, default_value = "50")]
    packet_size: u16,
    // Interval to wait between packets: Default is 0
    #[clap(short = 'i', long, default_value = "0")]
    interval: u64,
}

fn main() -> io::Result<()> {
    print!("\n");
    let args: Args = Args::parse();
    let connection_str: String;
    match args.ip {
        Some(ip) => {
            connection_str = format!("{}:{}", ip, args.port);
        },
        None => {
            let mut cmdhlp = Args::command();
            let _ = cmdhlp.print_help().unwrap();
            return Ok(());        }
    }
    

    // connect to the device
    let mut stream = TcpStream::connect(connection_str)?;


    // read the contents of the file into a buffer
    let mut contents = Vec::new();
    match args.file {
        Some(file_name) => {
            // read from file
            let mut file = File::open(file_name)?;
            file.read_to_end(&mut contents)?;
        },
        None => {
            // read from STDIN
            io::stdin().read_to_end(&mut contents)?;
        }
    }

    // Check if the last byte of the file is a newline character.
    let has_newline = contents.last().map(|&b| b == b'\n').unwrap_or(false);
    if !has_newline {
        // Append a newline character to the end of the file.
        contents.push(b'\n');
    }

    // Prints the number of bytes to be sent
    println!("Sending {} bytes", contents.len());
    let mut bytes_sent = 0;
    // send the buffer over the socket
    for chunk in contents.chunks(args.packet_size as usize) {
        stream.write_all(chunk)?;
        thread::sleep(Duration::from_millis(args.interval));
        // Prints bytes reamining.
        bytes_sent += chunk.len();
        print!("\rBytes sent: {} of {}", bytes_sent, contents.len());
    }

    Ok(())
}