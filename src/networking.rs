use {std::net::TcpListener, std::net::TcpStream};
use crate::common::{ClientRole, Guess, GuessResult};
use crate::io_utils::get_user_input;
use std::io::{Read, Write};

static PORT: i16 = 2000;

fn listen_for_peer() -> TcpStream {
    println!("Listening for incoming connections on port {}...", PORT);
    println!("Please make sure that your router is forwarding packets on this port.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT))
        .expect("Could not create TCP listener");

    let (stream, peer_addr) = listener.accept().unwrap();
    println!("Established connection with {}.", peer_addr);

    return stream;
}

fn connect_to_peer() -> TcpStream {
    let peer_addr = get_user_input("IP address to connect to:");
    println!("Attempting to connect to {} on port {}.", peer_addr, PORT);

    if let Ok(stream) = TcpStream::connect(format!("{}:{}", peer_addr, PORT)) {
        println!("Established connection with {}.", peer_addr);
        return stream;
    } else {
        panic!("Failed to establish connection with {}.", peer_addr);
    }
}

pub fn establish_connection() -> (ClientRole, TcpStream) {
    println!("Do you want to play the role of 'Word Master' or 'Guessing Player'?");
    let choice = get_user_input("Press 1 for Word Master or 2 for Guessing Player.");

    match choice.as_str() {
        "1" => return (ClientRole::WordMaster, listen_for_peer()),
        "2" => return (ClientRole::GuessingPlayer, connect_to_peer()),
        _ => panic!("Invalid option {}. Accepted values in [1, 2].", choice),
    }
}

pub fn send_solution_size(connection: &mut TcpStream, size: usize) {
    let buffer = size.to_le_bytes();
    connection.write(&buffer).expect("Cannot write to stream");
}

pub fn receive_solution_size(connection: &mut TcpStream) -> usize {
    let mut buffer: [u8; 8] = [0; 8];
    connection.read(&mut buffer).expect("Cannot read from stream");
    return usize::from_le_bytes(buffer);
}

pub fn receive_guess(connection: &mut TcpStream) -> Guess {
    let mut length_bytes = [0u8; 4];
    connection.read_exact(&mut length_bytes).unwrap();
    let length = u32::from_le_bytes(length_bytes) as usize;

    let mut buffer = vec![0; length];
    connection.read_exact(&mut buffer).unwrap();

    let result: Guess = bincode::deserialize(&buffer).unwrap();
    return result;
}

pub fn send_guess_result(connection: &mut TcpStream, guess_result: GuessResult) {
    let serialized_data = bincode::serialize(&guess_result).unwrap();

    let length = serialized_data.len() as u32;
    connection.write_all(&length.to_le_bytes()).unwrap();

    connection.write_all(&serialized_data).unwrap();
}

pub fn try_guess(connection: &mut TcpStream, guess: Guess) -> GuessResult {
    // Send the guess
    {
        let serialized_data = bincode::serialize(&guess).unwrap();

        let length = serialized_data.len() as u32;
        connection.write_all(&length.to_le_bytes()).unwrap();

        connection.write_all(&serialized_data).unwrap();
    }

    // Receive the guess result
    {
        let mut length_bytes = [0u8; 4];
        connection.read_exact(&mut length_bytes).unwrap();
        let length = u32::from_le_bytes(length_bytes) as usize;

        let mut buffer = vec![0; length];
        connection.read_exact(&mut buffer).unwrap();

        let result: GuessResult = bincode::deserialize(&buffer).unwrap();
        return result;
    }
}