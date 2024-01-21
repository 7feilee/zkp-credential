# ZKP Credential

## Overview

The ZKP Credential project is a Rust-based implementation showcasing zero-knowledge proof (ZKP) techniques for secure credential verification. The project consists of a client and a server, demonstrating the use of Bulletproofs for proving knowledge of a secret value within a specific range.

The primary use cases for this project include integrating it into a SOCKS 5 proxy, TCP Fast Open with cookie authentication, and Self-Sovereign Identity (SSI) systems. The goal is to provide a flexible and secure mechanism for proving credentials without revealing sensitive information.

## Features

- **Zero-Knowledge Proof (ZKP):** Utilizes Bulletproofs for proving knowledge of a secret value without disclosing the value itself.
- **Client-Server Architecture:** Demonstrates a basic client-server architecture where the client proves knowledge of a secret value to the server.
- **Use Cases:**
  - **SOCKS 5 Proxy Integration:** The ZKP credential system can be integrated into a SOCKS 5 proxy for secure authentication.
  - **TCP Fast Open with Cookie Auth:** Suitable for incorporating into TCP Fast Open with cookie-based authentication for efficient and secure connections.
  - **SSI (Self-Sovereign Identity):** The project can be extended for use in SSI systems where individuals can prove their identity without revealing sensitive details.


## Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/7feilee/zkp-credential.git
   cd zkp-credential
   ```

2. Build and run the project:

   ```bash
   cargo run
   ```

3. Follow the console output to observe the interaction between the client and server.

## Integration Guide

To integrate the ZKP Credential project into your specific use case, follow these steps:

1. **Modify Credential Values:**
   Adjust the `secret_value` and any other relevant parameters in `client_logic()` to match your credential requirements.

2. **Extend Server Logic:**
   Adapt the `server_logic()` function to incorporate additional verification steps or integrate with your specific application.

3. **Integrate with SOCKS 5 Proxy or TCP Fast Open:**
   Use the provided `send_proof_and_committed_value` function to send the proof and committed value securely. Adapt this logic to integrate with SOCKS 5 proxy authentication or TCP Fast Open with cookie authentication.

4. **SSI Integration:**
   Extend the project for use in Self-Sovereign Identity systems by incorporating additional cryptographic primitives or identity verification mechanisms.

## Contributors

- Qifei Li
- ChatGPT

## License

GPL-3.0 License. See [License here](./LICENSE) for details.

For more information about Bulletproofs and related libraries used in this project, refer to their respective licenses and documentation.