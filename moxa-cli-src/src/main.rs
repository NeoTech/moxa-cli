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
    // Receive a file from the remote device: Default is false
    #[clap(short = 'r', long, value_name = "BOOLEAN", default_value = "false")]
    receive: bool,
}


fn receive_file(mut stream: TcpStream) -> io::Result<()> {
    let args: Args = Args::parse();
    let mut contents = Vec::new();
    let mut buffer = [0; 1024];
    let mut bytes_received = 0;
    let mut idle_secs = 0;
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;
    loop {
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                contents.extend_from_slice(&buffer[..n]);
                bytes_received += n;
                println!("\rBytes received: {}", bytes_received);
                idle_secs = 1;
            },
            Ok(_) | Err(_) => {
                if idle_secs > 0 {
                    println!("\rClosing connection in 10s...");
                    io::stdout().flush().unwrap();
                    thread::sleep(Duration::from_secs(1));
                    idle_secs -= 1;
                } else {
                    println!("\nConnection closed.");
                    break;
                }
            },
        };
    }

    let file_name = args.file.as_ref().unwrap();
    let mut file = File::create(file_name)?;
    file.write_all(&contents)?;
    Ok(())
}

fn send_file(mut stream: TcpStream) -> io::Result<()> {
    let args: Args = Args::parse();
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

fn main() -> io::Result<()> {
    print!("\n");
    let args: Args = Args::parse();
    let connection_str: String;
    match args.ip {
        Some(ip) => {
            connection_str = format!("{}:{}", ip, args.port);
            if args.receive == false {
                // send the file to the remote device
                let stream = Some(TcpStream::connect(connection_str)?);
                send_file(stream.unwrap())?;
            } else {
                let listener = Some(TcpStream::connect(connection_str)?);
                receive_file(listener.unwrap())?;
            }
        },
        None => {
            let mut cmdhlp = Args::command();
            let _ = cmdhlp.print_help().unwrap();
            return Ok(());        
        }
    }
    Ok(())
}