use std::env;
use std::io;
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};
use std::process;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use serde_json::json;
use serde_json::Value;

fn main() {
    let socket = match UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 0))) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Failed to initiate server! Kind: {}, Info: {}",
                e.kind().to_string(),
                e.to_string()
            );
            process::exit(0);
        }
    };
    socket.set_write_timeout(Some(Duration::new(5, 0))).unwrap();
    match socket.connect(SocketAddr::from(([172, 20, 10, 10], 6666))) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "Failed to connect to server! Kind: {}, Info: {}",
                e.kind().to_string(),
                e.to_string()
            );
            process::exit(0);
        }
    }

    if let Err(e) = socket.send(json!({"func":1}).to_string().as_bytes()) {
        println!(
            "Failed to send data! Kind: {}, Info: {}",
            e.kind().to_string(),
            e.to_string()
        );
    }
    let (tx, rx) = mpsc::channel();
    let mut code: Option<String> = None;
    let recv_socket: UdpSocket = socket.try_clone().unwrap();
    recv_socket.set_write_timeout(Some(Duration::new(5, 0))).unwrap();
    let recv_thread: JoinHandle<()> = thread::spawn(move || {
        let mut raw_data: [u8; 2048] = [0u8; 2048];

        loop {
            let len = match recv_socket.recv(&mut raw_data) {
                Ok(l) => l,
                Err(e) => {
                    println!(
                        "Failed to receive data! Kind: {}, Info: {}",
                        e.kind().to_string(),
                        e.to_string()
                    );
                    return;
                }
            };

            let json_object: Value =
                serde_json::from_str(str::from_utf8(&raw_data[..len]).unwrap()).unwrap();

            match json_object.get("func").unwrap().as_i64().unwrap() {
                1 => {
                    println!("{}", json_object.get("info").unwrap().as_str().unwrap());
                }
                2 => {
                    println!("{}", json_object.get("info").unwrap().as_str().unwrap());
                    // //println!("hi");
                    // if json_object.get("rtn").unwrap().as_bool().unwrap() {
                    //     //println!("proper");
                    //     code = Some(
                    //         json_object
                    //             .get("code")
                    //             .unwrap()
                    //             .as_str()
                    //             .unwrap()
                    //             .to_string(),
                    //     );
                    //     //println!("{}", code.unwrap());
                    // }
                }
                3 => {
                    println!("{}", json_object.get("info").unwrap().as_str().unwrap());
                    if json_object.get("rtn").unwrap().as_bool().unwrap() {
                        process::exit(0);
                    }
                }
                4 => {
                    println!("{}", json_object.get("info").unwrap().as_str().unwrap());
                }
                5 => {
                    println!("{}", json_object.get("info").unwrap().as_str().unwrap());
                }
                6 => {
                    println!("{}", json_object.get("info").unwrap().as_str().unwrap());
                }
                7 => {
                    println!("{}", json_object.get("info").unwrap().as_str().unwrap());
                }
                100 => {
                    //println!("{}", code != None);
                    if json_object.get("rtn").unwrap().as_bool().unwrap() {
                        println!(
                            "[Private] {}[{}]: {}",
                            json_object.get("from").unwrap().as_str().unwrap(),
                            json_object.get("addr").unwrap().as_str().unwrap(),
                            json_object.get("content").unwrap().as_str().unwrap(),
                        );
                        tx.send(
                            json_object
                                .get("addr")
                                .unwrap()
                                .as_str()
                                .unwrap()
                                .to_string(),
                        )
                        .unwrap();
                    }
                }
                101 => {
                    if json_object.get("rtn").unwrap().as_bool().unwrap() {
                        println!(
                            "[Broadcast] {}[{}]: {}",
                            json_object.get("from").unwrap().as_str().unwrap(),
                            json_object.get("addr").unwrap().as_str().unwrap(),
                            json_object.get("content").unwrap().as_str().unwrap(),
                        );
                    }
                }
                _ => {
                    println!("{}", str::from_utf8(&raw_data[..len]).unwrap());
                }
            }

            raw_data = [0u8; 2048];
        }
    });

    let mut cin: String = String::new();
    let mut cmd: Vec<String> = Vec::new();
    let mut each: String = String::new();
    let mut quoted: bool = false;

    loop {
        io::stdin()
            .read_line(&mut cin)
            .expect("Failed to read line!");
        io::stdout().flush().unwrap();

        cin = cin.trim().to_string();
        if cin.is_empty() {
            continue;
        }

        for ch in cin.chars() {
            if ch == '"' {
                quoted = !quoted;
            } else if ch.is_whitespace() && !quoted && !each.is_empty() {
                cmd.push(each.clone());
                each.clear();
            } else {
                each.push(ch);
            }
        }
        if quoted {
            println!("Quote doesn't match!");
            continue;
        } else if !each.is_empty() {
            cmd.push(each.clone());
        }

        match cmd.get(0).unwrap().as_str() {
            "code" => {
                if cmd.len() != 2 {
                    println!("Unsupported Parameter Count!");
                    cin.clear();
                    cmd.clear();
                    quoted = false;
                    each.clear();
                    continue;
                }

                let code = cmd.get(1).unwrap();
                match socket.send(json!({"func":2,"code":code}).to_string().as_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!(
                            "Failed to send data! Kind: {}, Info: {}",
                            e.kind().to_string(),
                            e.to_string()
                        );
                    }
                }
            }
            "exit" => {
                if cmd.len() != 1 {
                    println!("Unsupported Parameter Count!");
                    cin.clear();
                    cmd.clear();
                    quoted = false;
                    each.clear();
                    continue;
                }
                socket
                    .send(json!({"func":3}).to_string().as_bytes())
                    .unwrap();
            }
            "sendmsg" => {
                if cmd.len() != 3 {
                    println!("Unsupported Parameter Count!");
                    cin.clear();
                    cmd.clear();
                    quoted = false;
                    each.clear();
                    continue;
                }

                let code = cmd.get(1).unwrap();
                let content = cmd.get(2).unwrap();

                socket
                    .send(
                        json!({"func":4,"code":code,"content":content})
                            .to_string()
                            .as_bytes(),
                    )
                    .unwrap();
            }
            "broadcast" => {
                if cmd.len() != 2 {
                    println!("Unsupported Parameter Count!");
                    cin.clear();
                    cmd.clear();
                    quoted = false;
                    each.clear();
                    continue;
                }

                let content = cmd.get(1).unwrap();
                socket
                    .send(json!({"func":5,"content":content}).to_string().as_bytes())
                    .unwrap();
            }
            "reply" => {
                if cmd.len() != 2 {
                    println!("Unsupported Parameter Count!");
                    cin.clear();
                    cmd.clear();
                    quoted = false;
                    each.clear();
                    continue;
                }

                let addr = match rx.try_recv() {
                    Ok(x) => x,
                    Err(e) => {
                        println!(
                            "No one recently sent you a message! Info: {}",
                            e.to_string()
                        );
                        continue;
                    }
                };
                let content = cmd.get(1).unwrap();
                socket
                    .send(
                        json!({"func":6,"addr":addr,"content":content})
                            .to_string()
                            .as_bytes(),
                    )
                    .unwrap();
            }
            "getusers" => {
                if cmd.len() != 1 {
                    println!("Unsupported Parameter Count!");
                    cin.clear();
                    cmd.clear();
                    quoted = false;
                    each.clear();
                    continue;
                }
                socket
                    .send(json!({"func":7}).to_string().as_bytes())
                    .unwrap();
            }
            _ => {
                println!("Command: {}; {:?}", cin, cmd);

                println!(
                    "Your input command {{{}}} has not been supported.",
                    cmd.get(0).unwrap()
                );
            }
        }

        cin.clear();
        cmd.clear();
        quoted = false;
        each.clear();
    }
}
