mod hash_table;
use hash_table::HashTable;

use std::sync::{Arc, Mutex};
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::io::BufRead;

#[derive(Debug)]
enum CommandFetchingState {
    WaitingForCommand,
    ReadingParameters { lines_to_read: u32 },
}

fn say_ok(mut stream: TcpStream){
    stream.write("+OK\r\n".as_bytes());
}
fn say_error(mut stream: TcpStream){
    stream.write("+Error\r\n".as_bytes());
}
fn command_set(key: String, value: String){

}

fn command_get(mut stream: TcpStream, key: String, hash_table: Arc<Mutex<HashTable>>){
    let result = hash_table.lock().unwrap().get(key.clone());
    match result {
        Some(value) => {
            let message = format!("+{:?}\r\n",value);
            stream.write(message.as_bytes()).unwrap();
        },
        None => {
            stream.write("+(nil)\r\n".as_bytes()).unwrap();
            return ()
        }
        // _ => ()
    }
}

fn handle_command(mut stream: TcpStream, commands: Vec<String>, hash_table: Arc<Mutex<HashTable>>){
    let root_command = &commands[0];
    // println!("{:?}",commands);
    match root_command.to_uppercase().as_str() {
        "COMMAND" => say_ok(stream),
        "SET" => {
            hash_table.lock().unwrap().set(commands[1].clone(), commands[2].clone());
            say_ok(stream);
        },
        "GET" => command_get(stream, commands[1].clone(), hash_table),
        _ => say_error(stream),
    }
    
}

fn handle_client(mut stream: TcpStream, mut hash_table: Arc<Mutex<HashTable>>) {
    let mut data = String::new();
    let mut reader = std::io::BufReader::new(&stream);
    let mut state = CommandFetchingState::WaitingForCommand;
    let mut command_buffer = Vec::new();
    while match reader.read_line(&mut data) {
        Ok(size) => {
            let char_vec: Vec<char> = data.chars().collect();
            data = String::new();

            match state {
                CommandFetchingState::WaitingForCommand => {
                    if char_vec[0] == '*'{
                        match char_vec[1].to_digit(10) {
                            Some(parameter_count) => {
                                state = CommandFetchingState::ReadingParameters{ 
                                    lines_to_read: parameter_count*2
                                };
                            },
                            None => {}
                        }
                    }
                },
                CommandFetchingState::ReadingParameters{lines_to_read} => {
                    if lines_to_read % 2 == 1 {
                        let mut command: String = char_vec.into_iter().collect();
                        let len = command.len();
                        command.truncate(len - 2);
                        command_buffer.push(command.clone());
                    }
                    if lines_to_read == 1 {
                        let writer_stream = stream.try_clone().expect("Cannot clone stream");
                        handle_command(writer_stream, command_buffer.clone(), hash_table.clone());
                        command_buffer = Vec::new();
                        state = CommandFetchingState::WaitingForCommand;
                    } else {
                        state = CommandFetchingState::ReadingParameters{
                            lines_to_read: lines_to_read - 1
                        };
                    }
                }
                _ => {}
            }

            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {

    // let mut hash_table = HashTable::new();
    let hash_table = Arc::new(Mutex::new(HashTable::new()));

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let cloned_hash_table = hash_table.clone();
                // println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream, cloned_hash_table)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}