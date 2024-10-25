/*mod database;
mod models;
mod api;

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use std::error::Error;
use std::time::Instant;
use log::info;
use serde_json::{json};
use crate::api::action::handle_message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    env_logger::init();
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
    let (mut write, mut read) = ws_stream.split();
    while let Some(Ok(msg)) = read.next().await {
        let start_time = Instant::now();
        if msg.is_text() {
            let text = msg.into_text().unwrap();
            match handle_message(text).await {
                Ok(response) => {
                    let json_response = serde_json::to_string(&response).unwrap();
                    write.send(Message::Text(json_response)).await.unwrap();
                    let duration = start_time.elapsed();
                    println!("Total Execution time: {:?}", duration);
                }
                Err(e) => {
                    let error_response = json!({"message":e.to_string(),});
                    let json_response = serde_json::to_string(&error_response).unwrap();
                    write.send(Message::Text(json_response)).await.unwrap();
                }
            }
        }
        else if msg.is_binary() {
            let binary_data = msg.into_data();
            println!("Received binary data: {:?}", binary_data);
            let response_data = process_binary_message(&binary_data).await; // Assume this processes the binary data
            match response_data {
                Ok(response) => {
                    write.send(Message::Binary(response)).await.unwrap();
                    let duration = start_time.elapsed();
                    println!("Total Execution time: {:?}", duration);
                }
                Err(e) => {
                    let error_response = json!({"message":e.to_string(),});
                    let json_response = serde_json::to_string(&error_response).unwrap();
                    write.send(Message::Binary(json_response.into_bytes())).await.unwrap();
                }
            }
        }
    }
}
pub async fn process_binary_message(data: &[u8]) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    //println!("data{:?}",data);
    let text = String::from_utf8(data.to_vec())?;
    //println!("text{}",text);
    let result_json = handle_message(text).await?;
    let result_binary = serde_json::to_vec(&result_json)?;
    Ok(result_binary)
}*/
/*mod database;
mod models;
mod api;

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use std::error::Error;
use std::time::Instant;
use log::info;
use serde_json::{json};
use crate::api::action::handle_message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    env_logger::init();
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
    let (mut write, mut read) = ws_stream.split();
    while let Some(Ok(msg)) = read.next().await {
        let start_time = Instant::now();
        if msg.is_text() {
            let text = msg.into_text().unwrap();
            let trimmed_str = text.trim_matches(['[', ']'].as_ref());
            let byte_values: Vec<u8> = trimmed_str.split(',').map(|s| s.trim().parse::<u8>()).collect::<Result<Vec<u8>, _>>().unwrap_or(Vec::from(""));
            let text = String::from_utf8(byte_values.to_vec()).unwrap_or("".parse().unwrap());
            match handle_message(text).await {
                Ok(response) => {
                    let json_response = serde_json::to_string(&response).unwrap();
                    let arr=json_response.into_bytes();
                    let string_rep = format!("{:?}", arr);
                    write.send(Message::Text(string_rep)).await.unwrap();
                    let duration = start_time.elapsed();
                    println!("Total Execution time: {:?}", duration);
                }
                Err(e) => {
                    let error_response = json!({"message":e.to_string(),});
                    let json_response = serde_json::to_string(&error_response).unwrap();
                    write.send(Message::Text(json_response)).await.unwrap();
                }
            }
        }
    }
}*/
mod database;
mod models;
mod api;

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use std::error::Error;
use std::time::Instant;
use log::info;
use serde_json::{json};
use crate::api::action::handle_message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    env_logger::init();
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(crate::handle_connection(stream));
    }
    Ok(())
}

// async fn handle_connection(stream: tokio::net::TcpStream) {
//     let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
//     let (mut write, mut read) = ws_stream.split();
//     while let Some(msg) = read.next().await {
//         match msg {
//             Ok(Message::Binary(data)) => {
//                 println!("Socket Req data: {:?}", data);
//                 let binary_data =data;
//                 let response_data = process_binary_message(&binary_data).await; // Assume this processes the binary data
//                 match response_data {
//                     Ok(response) => {
//                         println!("Socket Response data: {:?}", response);
//                         write.send(Message::Binary(response)).await.unwrap();
//                     }
//                     Err(e) => {
//                         let error_response = json!({"status":1,"message":e.to_string(),});
//                         let json_response = serde_json::to_string(&error_response).unwrap();
//                         write.send(Message::Binary(json_response.into_bytes())).await.unwrap();
//                     }
//                 }
//             }
//             Ok(_) => {
//                 let response = json!({"status":1,"message":"Invaild Format"});
//                 let json_response = serde_json::to_string(&response).unwrap();
//                 write.send(Message::Binary(json_response.into_bytes())).await.unwrap();
//             }
//             Err(e) => {
//                 let error_response = json!({"status":1,"message":e.to_string(),});
//                 let json_response = serde_json::to_string(&error_response).unwrap();
//                 write.send(Message::Binary(json_response.into_bytes())).await.unwrap();
//             }
//         }
//     }
// }
// pub async fn process_binary_message(data: &[u8]) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
//     let text = String::from_utf8(data.to_vec())?;
//     println!("Req data {}",text);
//     let result_json = handle_message(text).await?;
//     println!("Res data {}",result_json);
//     let result_binary = serde_json::to_vec(&result_json)?;
//     Ok(result_binary)
// }
async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                println!("Socket Req data: {:?}", data);
                match process_binary_message(&data).await {
                    Ok(response) => {
                        println!("Socket Response data: {:?}", response);
                        if let Err(e) = write.send(Message::Binary(response)).await {
                            eprintln!("Error sending response: {:?}", e);
                        }
                    }
                    Err(e) => {
                        let error_response = json!({"status": 1, "message": e.to_string()});
                        if let Ok(json_response) = serde_json::to_vec(&error_response) {
                            if let Err(e) = write.send(Message::Binary(json_response)).await {
                                eprintln!("Error sending error response: {:?}", e);
                            }
                        }
                    }
                }
            }
            Ok(_) => {
                let response = json!({"status": 1, "message": "Invalid Format"});
                if let Ok(json_response) = serde_json::to_vec(&response) {
                    if let Err(e) = write.send(Message::Binary(json_response)).await {
                        eprintln!("Error sending invalid format response: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {:?}", e);
                let error_response = json!({"status": 1, "message": e.to_string()});
                if let Ok(json_response) = serde_json::to_vec(&error_response) {
                    if let Err(e) = write.send(Message::Binary(json_response)).await {
                        eprintln!("Error sending error response: {:?}", e);
                    }
                }
            }
        }
    }
}

pub async fn process_binary_message(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let text = String::from_utf8(data.to_vec()).map_err(|e| format!("Invalid UTF-8 data: {}", e))?;
    println!("Req data: {}", text);
    let result_json = handle_message(text).await?;
    println!("Res data: {}", result_json);
    let result_binary = serde_json::to_vec(&result_json)?;
    Ok(result_binary)
}
