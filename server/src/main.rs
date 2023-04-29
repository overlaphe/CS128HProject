use std::collections::{HashMap, HashSet};
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::process;
use std::str;
use std::thread;
use std::thread::JoinHandle;

use serde_json::json;
use serde_json::Value;

struct Client {
    code: Option<String>,
    addr: SocketAddr,
    with: Option<Box<Client>>,
}
fn main() {
    let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 0))).unwrap();
    // match socket.connect(SocketAddr::from(([10, 191, 17, 248], 6666))) {
    //     Ok(_) => {}
    //     Err(_) => {
    //         println!("Failed to connect to server!");
    //         process::exit(0);
    //     }
    // }

    let recv_socket: UdpSocket = socket.try_clone().unwrap();
    let recv_thread: JoinHandle<()> = thread::spawn(move || {
        let mut used_codes:HashSet<String>=HashSet::new();
        let mut clients: Vec<Client> = Vec::new();
        let mut raw_data: [u8; 2048] = [0u8; 2048];

        loop {
            let (len, addr) = recv_socket.recv_from(&mut raw_data).unwrap();

            let json_object: Value =
                serde_json::from_str(str::from_utf8(&raw_data[..len]).unwrap()).unwrap();

            match json_object.get("func").unwrap().as_i64().unwrap() {
                2 => {
                    println!("==============================");
                    println!("Data Received From: {}", addr);
                    println!("Function: Set Client Code");

                    let client: &mut Client = clients
                        .iter_mut()
                        .find(|client| client.addr == addr)
                        .unwrap();

                    let code: String = json_object
                        .get("code")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();

                    if code.len() < 4 || code.len() > 12 {
                        recv_socket
                                    .send_to(
                                        json!({"func":1,"rtn":false,"info":"The code is too short or too long!"})
                                            .to_string()
                                            .as_bytes(),
                                        addr,
                                    )
                                    .unwrap();
                        println!("Return: False/Failed");
                        println!("Message: The code is too short or too long!");
                    } else {
                        (*client).code = Some(code.clone());
                        recv_socket
                                    .send_to(
                                        json!({"func":1,"rtn":true,"info":"Your code has successfully been set! You can wait for your friends!","code":code})
                                            .to_string()
                                            .as_bytes(),
                                        addr,
                                    )
                                    .unwrap();
                        println!("Return: True/Executed");
                        println!("Message: The code has successfully been set!");
                    }
                }
                _ => {}
            }
        }

        raw_data = [0u8; 2048];
    });
}
