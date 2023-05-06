use std::collections::HashSet;
use std::net::{SocketAddr, UdpSocket};
use std::str;
use std::thread;
use std::thread::JoinHandle;

use serde_json::json;
use serde_json::Value;

struct Client {
    code: Option<String>,
    addr: SocketAddr,
}
fn main() {
    let socket: UdpSocket = match UdpSocket::bind(SocketAddr::from(([172, 20, 10, 10], 6666))) {
        Ok(s) => s,
        Err(e) => {
            println!("UdpSocket::bind; Error: {}", e.to_string());
            return;
        }
    };

    let recv_socket: UdpSocket = socket.try_clone().unwrap();
    let recv_thread: JoinHandle<()> = thread::spawn(move || {
        let mut used_codes: HashSet<String> = HashSet::new();
        let mut clients: Vec<Client> = Vec::new();
        let mut raw_data: [u8; 2048] = [0u8; 2048];

        println!("Start receiving data from client!");

        loop {
            let (len, addr) = recv_socket.recv_from(&mut raw_data).unwrap();

            println!("{}", str::from_utf8(&raw_data[..len]).unwrap());

            let json_object: Value =
                serde_json::from_str(str::from_utf8(&raw_data[..len]).unwrap()).unwrap();

            match json_object.get("func").unwrap().as_i64().unwrap() {
                1 => {
                    println!("==============================");
                    println!("Data Received From: {}", addr);
                    println!("Function: Connect to Server");
                    match clients.iter().find(|client| client.addr == addr) {
                        Some(_v) => {
                            //recv_socket.send_to(, );
                        }
                        None => {
                            clients.push(Client {
                                code: None,
                                addr: addr,
                            });
                            recv_socket.send_to(json!({"func":1,"rtn":true,"info":format!("Successfully connected to server from {}",addr.to_string())}).to_string().as_bytes(), addr).unwrap();
                        }
                    }
                    println!("Return: True/Executed");
                    println!(
                        "Message: The client {} has successfully connected to our server!.",
                        addr.to_string()
                    );
                    println!("==============================");
                    println!();
                }
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
                                        json!({"func":2,"rtn":false,"info":"The code is too short or too long!"})
                                            .to_string()
                                            .as_bytes(),
                                        addr,
                                    )
                                    .unwrap();
                        println!("Return: False/Failed");
                        println!("Message: The code is too short or too long!");
                    } else {
                        if used_codes.insert(code.clone()) {
                            (*client).code = Some(code.clone());
                            recv_socket
                                        .send_to(
                                            json!({"func":2,"rtn":true,"info":format!("Your code {} has successfully been set!",code),"code":code})
                                                .to_string()
                                                .as_bytes(),
                                            addr,
                                        )
                                        .unwrap();
                            println!("Return: True/Executed");
                            println!("Message: The code has successfully been set! The address {} has code {}.",addr.to_string(),code);
                        } else {
                            recv_socket
                                        .send_to(
                                            json!({"func":2,"rtn":false,"info":"Your code has been used by another user! You can wait for your friends!","code":code})
                                                .to_string()
                                                .as_bytes(),
                                            addr,
                                        )
                                        .unwrap();
                            println!("Return: False/Failed");
                            println!("Message: The code has been used by another user.");
                        }
                    }
                    println!("==============================");
                    println!();
                }
                3 => {
                    println!("==============================");
                    println!("Data Received From: {}", addr);
                    println!("Function: Disconnect From Server");

                    clients.remove(
                        clients
                            .iter()
                            .enumerate()
                            .find(|(_, client)| client.addr == addr)
                            .map(|(i, _)| i)
                            .unwrap(),
                    );

                    // for client in &clients {
                    //     println!(
                    //         "{}:{}",
                    //         client.code.clone().unwrap(),
                    //         client.addr.clone().to_string()
                    //     );
                    // }

                    socket.send_to(json!({"func":3,"rtn":true,"info":"You've successfully disconnected from the server! Have a good day."}).to_string().as_bytes(), addr).unwrap();

                    println!("Return: True/Executed");
                    println!("Message: Disconnect successfully!");
                    println!("==============================");
                    println!();
                }
                4 => {
                    println!("==============================");
                    println!("Data Received From: {}", addr);
                    println!("Function: Send Private Message");

                    if clients.len() < 2 {
                        socket.send_to(json!({"func":4,"rtn":false,"info":"Few than 2 people connect to the server, please wait for other users connect to the server."}).to_string().as_bytes(), addr).unwrap();

                        println!("Return: False/Failed");
                        println!("Message: Few than 2 people connect to the server, please wait for other users connect to the server.");
                        println!("==============================");
                        println!();

                        continue;
                    }

                    let code: String = json_object
                        .get("code")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                    let content: String = json_object
                        .get("content")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();

                    if let Some(receiver) = clients
                        .iter()
                        .find(|client| client.code == Some(code.clone()))
                    {
                        let sender = clients.iter().find(|client| client.addr == addr).unwrap();
                        if sender.code.eq(&receiver.code) {
                            socket.send_to(json!({"func":4,"rtn":false,"info":"You cannot send message to yourself."}).to_string().as_bytes(), addr).unwrap();

                            println!("Return: False/Failed");
                            println!("Message: Users cannot send message to themselves!");
                            println!("==============================");
                            println!();
                        } else {
                            match socket.send_to(json!({"func":100,"rtn":true,"from":sender.code.clone().unwrap_or("[[Anonymous]]".to_string()),"addr":sender.addr.to_string(),"content":content}).to_string().as_bytes(), receiver.addr){
                                Ok(_)=>{
                                    socket.send_to(json!({"func":4,"rtn":true,"info":"Sent"}).to_string().as_bytes(), addr).unwrap();

                                    println!("Return: True/Executed");
                                    println!("Message: Sent!");
                                    println!("==============================");
                                    println!();
                                },
                                Err(_)=>{
                                    socket.send_to(json!({"func":4,"rtn":false,"info":format!("Failed to send message to your friend {}!",receiver.code.clone().unwrap())}).to_string().as_bytes(), addr).unwrap();

                                    println!("Return: False/Failed");
                                    println!("Message: Failed to send message to your friend {}!",receiver.code.clone().unwrap());
                                    println!("==============================");
                                    println!();
                                }
                            }
                        }
                    } else {
                        socket.send_to(json!({"func":4,"rtn":false,"info":"The code is not found. Please correctly input the code!"}).to_string().as_bytes(), addr).unwrap();

                        println!("Return: False/Failed");
                        println!(
                            "Message: The code is not found. Please correctly input the code!"
                        );
                        println!("==============================");
                        println!();
                    }
                }
                5 => {
                    println!("==============================");
                    println!("Data Received From: {}", addr);
                    println!("Function: Broadcast Message");

                    if clients.len() < 2 {
                        socket.send_to(json!({"func":5,"rtn":false,"info":"Few than 2 people connect to the server, please wait for other users connect to the server."}).to_string().as_bytes(), addr).unwrap();

                        println!("Return: False/Failed");
                        println!("Message: Few than 2 people connect to the server, please wait for other users connect to the server.");
                        println!("==============================");
                        println!();

                        continue;
                    }

                    let content: String = json_object
                        .get("content")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();

                    let sender = clients.iter().find(|client| client.addr == addr).unwrap();

                    let mut sent_all: bool = true;
                    for client in &clients {
                        if client.addr == sender.addr {
                            continue;
                        }

                        if let Err(_) = socket.send_to(
                            json!({"func":101,"rtn":true,"from":sender.code.clone().unwrap_or("[[Anonymous]]".to_string()),"addr":sender.addr.to_string(),"content":content}).to_string().as_bytes(),
                            client.addr,
                        ) {
                            sent_all = false;
                        }
                    }

                    if sent_all {
                        socket
                            .send_to(
                                json!({"func":5,"rtn":true,"info":"Sent"})
                                    .to_string()
                                    .as_bytes(),
                                addr,
                            )
                            .unwrap();

                        println!("Return: True/Executed");
                        println!("Message: Sent!");
                        println!("==============================");
                        println!();
                    } else {
                        socket
                            .send_to(
                                json!({"func":5,"rtn":true,"info":"Failed to send message to some users!"})
                                    .to_string()
                                    .as_bytes(),
                                addr,
                            )
                            .unwrap();

                        println!("Return: False/Failed");
                        println!("Message: Failed to send message to some users!");
                        println!("==============================");
                        println!();
                    }
                }
                6 => {
                    println!("==============================");
                    println!("Data Received From: {}", addr);
                    println!("Function: Reply Message");

                    let content = json_object
                        .get("content")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();

                    let sender: &Client =
                        clients.iter().find(|client| client.addr == addr).unwrap();
                    let receiver_addr = json_object
                        .get("addr")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .parse::<SocketAddr>()
                        .unwrap();
                    match clients.iter().find(|client| client.addr == receiver_addr) {
                        None => {
                            socket.send_to(json!({"func":6,"rtn":false,"info":"Address not found now! He/She might be offline now."}).to_string().as_bytes(), addr).unwrap();

                            println!("Return: False/Failed");
                            println!("Message: Address not found!");
                            println!("==============================");
                            println!();
                        }
                        Some(client) => {
                            socket.send_to(json!({"func":100,"rtn":true,"from":sender.code.clone().unwrap_or("[[Anonymous]]".to_string()),"addr":sender.addr.to_string(),"content":content}).to_string().as_bytes(), client.addr).unwrap();
                            socket
                                .send_to(
                                    json!({"func":6,"rtn":true,"info":"Sent"})
                                        .to_string()
                                        .as_bytes(),
                                    addr,
                                )
                                .unwrap();

                            println!("Return: True/Executed");
                            println!("Message: Replied!");
                            println!("==============================");
                            println!();
                        }
                    }
                }
                7 => {
                    println!("==============================");
                    println!("Data Received From: {}", addr);
                    println!("Function: Get User List");

                    // let mut json = String::new();
                    // json.push('[');

                    // for i in 0..clients.len() {
                    //     json.push('{');
                    //     json.push_str("\"code\":");
                    //     json.push('"');
                    //     json.push_str(
                    //         clients
                    //             .get(i)
                    //             .unwrap()
                    //             .code
                    //             .clone()
                    //             .unwrap_or("[[Anonymous]]".to_string())
                    //             .as_str(),
                    //     );
                    //     json.push('"');
                    //     json.push(',');
                    //     json.push_str("\"addr\":");
                    //     json.push('"');
                    //     json.push_str(clients.get(i).unwrap().addr.to_string().as_str());
                    //     json.push('"');
                    //     json.push('}');
                    //     if i != clients.len() - 1 {
                    //         json.push(',');
                    //     }
                    // }
                    // json.push(']');

                    let mut info = String::new();
                    for i in 0..clients.len() {
                        info.push_str(
                            format!(
                                "{}[{}]",
                                clients
                                    .get(i)
                                    .unwrap()
                                    .code
                                    .clone()
                                    .unwrap_or("[[Anonymous]]".to_string()),
                                clients.get(i).unwrap().addr.to_string(),
                            )
                            .as_str(),
                        );

                        if clients.get(i).unwrap().addr == addr {
                            info.push_str("(YOU)");
                        }

                        if i != clients.len() - 1 {
                            info.push('\n');
                        }
                    }

                    socket
                        .send_to(
                            json!({"func":7,"rtn":true,"info":info})
                                .to_string()
                                .as_bytes(),
                            addr,
                        )
                        .unwrap();

                    println!("Return: True/Executed");
                    println!("Message: Returned!");
                    println!("==============================");
                    println!();
                }
                _ => {}
            }

            raw_data = [0u8; 2048];
        }
    });

    // let mut cin=String::new();
    // loop {
    //     io::stdin()
    //         .read_line(&mut cin)
    //         .expect("Failed to read line!");
    //     io::stdout().flush().unwrap();

    //     match cin.as_str() {
    //         "exit"=>{
    //             process::exit(0);
    //         }
    //         &_=>{

    //         }
    //     }
    // }

    recv_thread.join().unwrap();
}
