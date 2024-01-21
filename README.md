# ZKP Credential

## Overview

This Rust project provides a basic example of using zero-knowledge proofs for secure credential verification between a client and a server. The project consists of a client and a server, demonstrating the use of Bulletproofs for proving knowledge of a secret value within a specific range ([0, 2^32)). In this generalized use case, the client's secret is considered as an elliptic curve private key, showcasing potential membership in a specific group. The binding used in the project plays a crucial role in achieving unlinkability.

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

## License

GPL-3.0 License. See [License here](./LICENSE) for details.

For more information about Bulletproofs and related libraries used in this project, refer to their respective licenses and documentation.