```markdown
# CipherGuard2

## Description

CipherGuard2 is a Rust-based library designed to provide a secure and efficient cryptographic toolkit for various applications. It offers a range of functionalities, from symmetric and asymmetric encryption to hashing and key derivation, all implemented with a focus on performance and adherence to modern cryptographic standards. The library aims to simplify the integration of robust security measures into your projects by providing a clean and well-documented API. CipherGuard2 prioritizes memory safety and leverages Rust's ownership system to minimize the risk of common cryptographic vulnerabilities.

## Features

*   **Symmetric Encryption:** Implements AES-256-GCM for authenticated encryption. This provides confidentiality and integrity, ensuring data is both encrypted and tamper-proof.
*   **Asymmetric Encryption:** Offers RSA encryption and decryption with OAEP padding, suitable for key exchange and digital signatures.
*   **Hashing:** Provides SHA-256 and SHA-3 hashing algorithms for data integrity checks and password storage (when combined with salting).
*   **Key Derivation:** Includes PBKDF2 for securely deriving encryption keys from passwords, making brute-force attacks more difficult.
*   **Secure Random Number Generation:** Utilizes the `rand` crate for cryptographically secure random number generation, essential for key generation and initialization vectors.

## Installation

To use CipherGuard2 in your Rust project, you'll need to add it as a dependency in your `Cargo.toml` file.

1.  **Add the dependency:**

    Open your `Cargo.toml` file and add the following line to the `[dependencies]` section:

    ```toml
    [dependencies]
    cipherguard2 = "0.1.0" # Replace with the actual version number
    ```

2.  **Ensure you have Rust and Cargo installed:**

    If you don't have Rust and Cargo installed, you can download them from the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

3.  **Build your project:**

    Run the following command in your project directory to build your project and download the dependencies:

    ```bash
    cargo build
    ```

4.  **Optional: Run tests:**

    To ensure that CipherGuard2 is working correctly, you can run the tests:

    ```bash
    cargo test
    ```

## Usage

Here are some examples of how to use CipherGuard2 in your Rust code.

**Symmetric Encryption (AES-256-GCM):**

```rust
use cipherguard2::symmetric::aes_gcm::{encrypt, decrypt, generate_key};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a random key
    let key = generate_key();

    // The data to encrypt
    let plaintext = b"This is a secret message.";

    // Encrypt the data
    let (ciphertext, nonce) = encrypt(&key, plaintext)?;

    // Decrypt the data
    let decrypted_plaintext = decrypt(&key, &ciphertext, &nonce)?;

    // Verify that the decrypted data matches the original plaintext
    assert_eq!(plaintext.to_vec(), decrypted_plaintext);

    println!("Original: {}", String::from_utf8_lossy(plaintext));
    println!("Encrypted: {:?}", ciphertext);
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted_plaintext));

    Ok(())
}
```

**Asymmetric Encryption (RSA):**

```rust
use cipherguard2::asymmetric::rsa::{generate_keypair, encrypt, decrypt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate an RSA key pair
    let (public_key, private_key) = generate_keypair(2048)?;

    // The data to encrypt
    let plaintext = b"This is a secret message.";

    // Encrypt the data using the public key
    let ciphertext = encrypt(&public_key, plaintext)?;

    // Decrypt the data using the private key
    let decrypted_plaintext = decrypt(&private_key, &ciphertext)?;

    // Verify that the decrypted data matches the original plaintext
    assert_eq!(plaintext.to_vec(), decrypted_plaintext);

    println!("Original: {}", String::from_utf8_lossy(plaintext));
    println!("Encrypted: {:?}", ciphertext);
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted_plaintext));

    Ok(())
}
```

**Hashing (SHA-256):**

```rust
use cipherguard2::hashing::sha256::hash;

fn main() {
    let data = b"This is some data to hash.";
    let hash_result = hash(data);
    println!("SHA-256 Hash: {:?}", hash_result);
}
```

**Key Derivation (PBKDF2):**

```rust
use cipherguard2::key_derivation::pbkdf2::derive_key;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let password = b"my_secret_password";
    let salt = b"random_salt"; // Should be unique and randomly generated
    let iterations = 100000; // Number of iterations (higher is better)
    let key_length = 32; // Length of the derived key in bytes

    let derived_key = derive_key(password, salt, iterations, key_length)?;

    println!("Derived Key: {:?}", derived_key);

    Ok(())
}
```

## Contributing

We welcome contributions to CipherGuard2! Here are some guidelines to follow:

1.  **Fork the repository:** Create your own fork of the CipherGuard2 repository on GitHub.
2.  **Create a branch:** Create a new branch for your changes. Use a descriptive name for your branch, such as `feature/new-encryption-algorithm` or `bugfix/aes-padding-issue`.
3.  **Make your changes:** Implement your changes, ensuring that they are well-documented and follow the existing code style.
4.  **Write tests:** Write unit tests to ensure that your changes are working correctly and that they don't introduce any regressions.
5.  **Run tests:** Run all the tests to make sure that your changes haven't broken any existing functionality.
6.  **Commit your changes:** Commit your changes with a clear and concise commit message.
7.  **Push your changes:** Push your branch to your forked repository on GitHub.
8.  **Create a pull request:** Create a pull request from your branch to the main branch of the CipherGuard2 repository.
9.  **Code Review:** Address any feedback provided during code review.
10. **Merge:** Once your pull request has been approved, it will be merged into the main branch.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/jjfhwang/CipherGuard2/blob/main/LICENSE) file for details.
```