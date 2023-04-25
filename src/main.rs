use std::io;
use std::io::Write;
use std::net::{UdpSocket,SocketAddr};
use std::process;
use std::str;
use std::thread;
use std::sync::{Arc,Mutex};

use serde_json;
use serde_json::json;
use serde_json::Value;

fn main() {
    let socket = match UdpSocket::bind(SocketAddr::from(([10,200,0,134],4567))) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to bind socket! {}", e.to_string());
            process::exit(0);
        }
    };

    match socket.connect(SocketAddr::from(([10,200,0,134],4567))) {
        Err(e) => {
            println!("Failed to connect to server! {}", e.to_string());
            process::exit(0);
        }
        Ok(_) => {}
    }


    let socket_cloned=socket.try_clone().unwrap();
    let thread: thread::JoinHandle<()> = thread::spawn(move || {
        let mut raw_data: [u8; 2048] = [0u8; 2048];
        loop {
            let (len, addr) = match socket_cloned.recv_from(&mut raw_data) {
                Ok((len, addr)) => (len, addr),
                Err(e) => {
                    println!("Failed to start receiving data from server!");
                    println!("Error msg: {}", e.to_string());
                    break;
                }
            };

            let json_object: Value = serde_json::from_str(match str::from_utf8(&raw_data[..len]) {
                Ok(str) => str,
                Err(e) => {
                    println!("Failed to parse UTF8 data!");
                    println!("Error msg: {}", e.to_string());
                    break;
                }
            })
            .unwrap();

            // TODO: Parse Json, change variables, etc.
        }
    });

    {
        println!("╔════════════════╦═══════════════════════════════════════════════╗");
        println!("║ cmd            ║ show this list                                ║");
        println!("╠════════════════╬═══════════════════════════════════════════════╣");
        println!("║ exit           ║ deconnect from server and exit the client.    ║");
        println!("╠════════════════╬═══════════════════════════════════════════════╣");
        println!("║ code [code]    ║ set your identifier as [code], other user can ║");
        println!("║                ║ use [code] to send something to you.          ║");
        println!("╠════════════════╬═══════════════════════════════════════════════╣");
        println!("║ bind [code]    ║ bind the user with [code]                     ║");
        println!("║                ║ if the server find [code], then they can.     ║");
        println!("║                ║ start chatting with each other.               ║");
        println!("╠════════════════╬═══════════════════════════════════════════════╣");
        println!("║ unbind         ║ unbind the current user!                      ║");
        println!("║                ║ You must currently bind with another user!    ║");
        println!("╠════════════════╬═══════════════════════════════════════════════╣");
        println!("║ sendmsg [msg]  ║ Please bind with another user at first!       ║");
        println!("║                ║ if the server find [code], then they can.     ║");
        println!("║                ║ start chatting with each other.               ║");
        println!("╚════════════════╩═══════════════════════════════════════════════╝");
    }

    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        io::stdout().flush().unwrap();

        if input.is_empty() {
            println!("Your command: {{{}}}", input);
            println!("Error: Your command is empty!");
            continue;
        }

        let split: Vec<String> = input.split_whitespace().map(String::from).collect();
        match split.get(0) {
            Some(command) => match command.as_str() {
                "cmd" => {
                    println!("╔════════════════╦═══════════════════════════════════════════════╗");
                    println!("║ cmd            ║ show this list                                ║");
                    println!("╠════════════════╬═══════════════════════════════════════════════╣");
                    println!("║ exit           ║ deconnect from server and exit the client.    ║");
                    println!("╠════════════════╬═══════════════════════════════════════════════╣");
                    println!("║ code [code]    ║ set your identifier as [code], other user can ║");
                    println!("║                ║ use [code] to send something to you.          ║");
                    println!("╠════════════════╬═══════════════════════════════════════════════╣");
                    println!("║ bind [code]    ║ bind the user with [code]                     ║");
                    println!("║                ║ if the server find [code], then they can.     ║");
                    println!("║                ║ start chatting with each other.               ║");
                    println!("╠════════════════╬═══════════════════════════════════════════════╣");
                    println!("║ unbind         ║ unbind the current user!                      ║");
                    println!("║                ║ You must currently bind with another user!    ║");
                    println!("╠════════════════╬═══════════════════════════════════════════════╣");
                    println!("║ sendmsg [msg]  ║ Please bind with another user at first!       ║");
                    println!("║                ║ if the server find [code], then they can.     ║");
                    println!("║                ║ start chatting with each other.               ║");
                    println!("╚════════════════╩═══════════════════════════════════════════════╝");
                }
                "exit" => {
                    todo!();
                    break;
                }
                "code" => {
                    if split.len() != 2 {
                        println!("Your command: {{{}}}", input);
                        println!("Error: Please indicate the code you'd like to set");
                    }

                    println!(
                        "{}",
                        json!({
                            "msg":0x0001,
                            "code":split.get(1).unwrap().to_string(),
                        })
                        .to_string()
                    );

                    socket.send(
                        json!({
                            "msg":0x0001,
                            "code":split.get(1).unwrap().to_string(),
                        })
                        .to_string()
                        .as_bytes(),
                    );
                }

                &_ => todo!(),
            },
            None => {
                println!("Your command: {{{}}}", input);
                println!("Error: Your command is empty!");
            }
        }
    }
}
// ╔════╦══════╦══╗
// ╠═══╗║╠═══╦╗║╚╗║
// ║╔═╗╬╠═╦═╣║║╚╚║║
// ║╚╗╠╚╦║║╔═╣╚═╦╝║
// ║║║╝╔═╩╝║╠╝╣╔╝╔╣
// ║║╚═╣╔══╣╔═╝║╩╣║
// ║╚═╗╚╝╣║╝║╝═╩╔╩║
// ╚══╩══╩══╩═════╝
