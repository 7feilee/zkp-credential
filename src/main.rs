use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek_ng::ristretto::CompressedRistretto;
use curve25519_dalek_ng::scalar::Scalar;
use rand::thread_rng;
use merlin::Transcript;
use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread::{self, sleep};
use std::time::Duration;

// Client logic
fn client_logic() {
    let secret_value = 0xdeadbeefu64;
    let binding = Scalar::random(&mut thread_rng());
    let mut prover_transcript = Transcript::new(b"zkp credential example");

    let (proof, committed_value) = RangeProof::prove_single(
        &BulletproofGens::new(256, 1),
        &PedersenGens::default(),
        &mut prover_transcript,
        secret_value,
        &binding,
        32,
    )
    .expect("error handling");

    // Send proof and committed value to the server
    println!("Client: Sending proof and committed value to the server");
    send_proof_and_committed_value(proof, committed_value);
}

// Server logic
fn server_logic(mut stream: TcpStream) {
    let mut verifier_transcript = Transcript::new(b"zkp credential example");

    // Receive serialized proof and committed value from the client
    let mut serialized_data: Vec<u8> = Vec::new();

    stream
        .read_to_end(&mut serialized_data)
        .expect("Read failed");

    println!("Server: Received serialized proof and committed value");

    // Split the received data into proof and committed value
    let (serialized_proof, serialized_committed_value) = serialized_data.split_at(serialized_data.len() - 32);

    // Deserialize proof and committed value
    let proof = RangeProof::from_bytes(&serialized_proof).expect("Deserialization error");
    let committed_value = CompressedRistretto::from_slice(&serialized_committed_value);

    assert!(
        proof
            .verify_single(
                &BulletproofGens::new(256, 1),
                &PedersenGens::default(),
                &mut verifier_transcript,
                &committed_value,
                32,
            )
            .is_ok()
    );

    // Log the verification result
    println!("Server: Proof verification successful");
}

// Implement RPC logic here to send proof and committed value to the server
fn send_proof_and_committed_value(proof: RangeProof, committed_value: CompressedRistretto) {
    // Implement RPC logic to send serialized proof and committed value to the server
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:8888").expect("Connection failed");
    let serialized_proof = proof.to_bytes();
    let serialized_committed_value = committed_value.to_bytes();

    stream
        .write_all(&serialized_proof)
        .expect("Client: Write serialized_proof failed");

    stream
        .write_all(&serialized_committed_value)
        .expect("Client: Write serialized_committed_value failed");

    // Log the sent proof and committed value
    println!("Client: Proof and committed value sent to the server");
    stream.shutdown(Shutdown::Write).expect("Shutdown failed");
}

fn main() {
    // Start the server in a separate thread
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:8888").expect("Failed to bind");
        println!("Server: Server is running and waiting for connections...");

        loop {
            println!("Server: Checking for incoming connections...");

            match listener.accept() {
                Ok((stream, _)) => {
                    println!("Server: Received a connection!");

                    // Handle each client connection in a separate thread
                    thread::spawn(move || {
                        server_logic(stream);
                    });
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            }
        }
    });

    client_logic();
    sleep(Duration::from_secs(3));
}
