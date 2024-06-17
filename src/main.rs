use std::os::unix::net::UnixListener;
use std::io::{Read, Write};
use std::fs;
use std::path::Path;
mod recover_wallet;
mod generate_wallet;
use std::thread;

fn handle_recover_wallet(socket_path: &str) -> std::io::Result<()> {
    let listener = UnixListener::bind(socket_path)?;
    println!("Server listening on socket: {}", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(mut socket) => {
                let addr = socket.peer_addr().unwrap();
                println!("Got a client on {}: {:?}", socket_path, addr);

                let mut buffer = [0; 1024];
                let bytes_read = socket.read(&mut buffer)?;
                let response = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
                let public_key = recover_wallet::recover_wallet(response);
                socket.write_all(public_key.as_bytes())?;
            },
            Err(e) => {
                println!("Failed to accept client connection on {}: {}", socket_path, e);
            }
        }
    }
    Ok(())
}

fn handle_generate_wallet(socket_path: &str) -> std::io::Result<()> {
    let listener = UnixListener::bind(socket_path)?;
    println!("Server listening on socket: {}", socket_path);
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut socket) => {
                let addr = socket.peer_addr().unwrap();
                println!("Got a client on {}: {:?}", socket_path, addr);

                let mut buffer = [0; 1024];
                let bytes_read = socket.read(&mut buffer)?;
                let response = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
                let recovery_length: u16 = response.to_string().parse().expect("Not a valid number");
                if recovery_length == 12 || recovery_length == 24 {
                    let (public_key, mnemonic) = generate_wallet::generate_wallet(recovery_length);
                    let data = &[(hex::encode(public_key.to_bytes()) + " / ").as_bytes(), mnemonic.to_string().as_bytes()].join(&[0, 0][..]);
                    socket.write_all(data)?;

                    // println!("Public Key: 0x{}", hex::encode(public_key.to_bytes()));
                    // println!("Mnemonic: {}", mnemonic.to_string());
                }
            },
            Err(e) => {
                println!("Failed to accept client connection on {}: {}", socket_path, e);
            }
        }
    }
    Ok(())
}


fn main() -> std::io::Result<()> {
    let socket_path1 = "/tmp/rst1.sock";
    let socket_path2 = "/tmp/rst2.sock";

    // Ensure the socket files do not already exist
    for &socket_path in &[socket_path1, socket_path2] {
        if Path::new(socket_path).exists() {
            fs::remove_file(socket_path)?;
        }
    }

    let thread_handle1 = thread::spawn(move || {
        handle_recover_wallet(socket_path1).unwrap_or_else(|err| {
            eprintln!("Error handling connections on {}: {}", socket_path1, err);
        });
    });

    let thread_handle2 = thread::spawn(move || {
        handle_generate_wallet(socket_path2).unwrap_or_else(|err| {
            eprintln!("Error handling connections on {}: {}", socket_path2, err);
        });
    });

    thread_handle1.join().expect("Thread 1 panicked");
    thread_handle2.join().expect("Thread 2 panicked");

    Ok(())
}
