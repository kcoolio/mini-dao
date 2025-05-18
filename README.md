# mini-dao
A beginner-friendly DAO voting smart contract built using Soroban SDK. Features user registration, proposal creation, token-based voting, and vote tallying with secure storage and unit tests.

# 🗳️ Soroban MINI-DAO Voting Contract

A simple, modular smart contract built using the [Soroban SDK](https://soroban.stellar.org/docs), designed to simulate a basic DAO (Decentralized Autonomous Organization) with token-based voting.

This project aims to demonstrate core concepts like user registration, proposal lifecycle, voting, and storage management.

---

## ✨ Features

- ✅ User registration with initial DAO token allocation
- ✅ Proposal creation with deadlines
- ✅ Voting using DAO tokens (1 token = 1 vote)
- ✅ One vote per user per proposal
- ✅ Vote tallying and results
- ✅ Modular code structure with basic error handling
- ✅ Unit tests for registration, voting, and results

---

## 📦 Project Structure

```

mini-dao\_contract/
├── Cargo.toml         # Rust and Soroban project config
└── src/
├── lib.rs         # Main contract logic
└── test.rs        # Unit tests

````

---

## 🚀 Getting Started

### 🔧 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Soroban CLI (optional)](https://soroban.stellar.org/docs/getting-started/installation)

### 🔨 Build the Contract

```bash
stellar contract build
````

The compiled WASM file will be located at:

```
target/wasm32-unknown-unknown/release/mini-dao.wasm
```

---

## 🧪 Running Tests

To run all unit tests:

```bash
cargo test
```

---

## 💻 Example Usage

### ➕ Register a User

```rust
mini-dao::register(env.clone(), user_address);
```

Gives the new user an initial balance of DAO tokens.

---

### 💰 Get User Balance

```rust
mini-dao::get_balance(env.clone(), user_address);
```

Returns the user's DAO token balance.

---

### 🗳️ Proposal and Voting (Coming Soon)

* `create_proposal`
* `submit_proposal`
* `vote_on_proposal`
* `get_results`

---

## 🧱 Built With

* [Rust](https://www.rust-lang.org/)
* [Soroban SDK](https://github.com/stellar/soroban-sdk)
* [Stellar Smart Contracts](https://soroban.stellar.org/docs)

---

## 📄 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## 🤝 Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you'd like to change.

---

## 📫 Contact

For support or feedback, please open a [GitHub Issue](https://github.com/kcoolio/mini-dao/issues).

```

```


