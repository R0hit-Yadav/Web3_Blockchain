# Ethereum Wallet & Smart Contract Interaction in Rust

This project demonstrates how to create an Ethereum wallet, check balances, send transactions, and 
interact with a smart contract using Rust and the `ethers-rs` and `web3` crates.

## 🚀 Features
- Generate a new Ethereum wallet with a private key.
- Check the wallet's balance.
- Send ETH from a sender wallet to a recipient.
- Interact with a smart contract to store and retrieve values.
- Fetch and display transaction receipts.

## 📦 Dependencies
Ensure you have Rust installed. You need the following Rust crates:

```toml
[dependencies]
ethers = "2.0"
web3 = "0.18"
dotenv = "0.15"
hex = "0.4"
tokio = { version = "1", features = ["full"] }
```

## 🛠 Setup
Clone the repository:

```sh
git clone https://github.com/R0hit-Yadav/Web3_Blockchain.git
cd Web3_Blockchain
```

Create a `.env` file in the project root and add the following:

```ini
ETHEREUM_RPC_URL="https://holesky.infura.io/v3/YOUR_INFURA_PROJECT_ID"
SENDER_PRIVATE_KEY="your_sender_wallet_private_key"
CONTRACT_ADDRESS="your_smart_contract_address"
```

## ▶️ Compile and Run

```sh
cargo run
```

## 📜 How It Works

### 1️⃣ Create a Wallet
- Generates a new Ethereum wallet.
- Displays the wallet address and private key.

### 2️⃣ Connect to an Ethereum Node
- Uses `ETHEREUM_RPC_URL` from `.env` to connect to an Ethereum provider.

### 3️⃣ Check Wallet Balance
- Fetches and prints the balance of the generated wallet.

### 4️⃣ Send an ETH Transaction
- Sends ETH from `SENDER_PRIVATE_KEY` to the generated wallet.
- Uses gas estimation to ensure enough balance is available.

### 5️⃣ Get Transaction Receipt
- Waits for transaction confirmation and displays receipt details.

### 6️⃣ Smart Contract Interaction
- Calls a contract function to store a value (`store(100)`).
- Retrieves the stored value from the contract.

## 🧠 What You Will Learn
- How to generate Ethereum wallets using `ethers-rs`.
- Connecting Rust to an Ethereum node using Web3 providers.
- Fetching account balances and handling gas estimation.
- Sending transactions programmatically with `ethers-rs`.
- Interacting with smart contracts via ABI encoding.

## ⚡ Example Output
```yaml
New Ethereum Wallet Address: 0x1f858a8e0f638d19445a532c2f61b15e48cb649f
Private Key: "7fc26dee638a4963f43deb5a431067bd76b0d67094981e3ea8ded68bd2fab209"

Balance New Wallet: 0.000000000000000000 ETH
Balance Of Sender: 4.199323392839830016 ETH
Transaction sent! Hash: 0x4a428912df27e2733280d549a293594c1c31d9f7214c6d696e057ddb13423b41

Transaction Receipt Generating....
=============================
||==>Transaction Receipt:<===
=============================
||  Transaction Index: 12
||  Transaction Hash: 0x4a428912df27e2733280d549a293594c1c31d9f7214c6d696e057ddb13423b41
||  Block Number: 3388923
||  From: 0xb53d942b79ab480e029220f768cf7b1629acfcf4
||  To: 0x1f858a8e0f638d19445a532c2f61b15e48cb649f
||  Gas Used: 21000
||  Status: "Success"
=============================

Interacting with the smart contract...
Stored Value Transaction Hash: 0x4193618433cf502f1a84eb37d9b6110128df4f008de86ed4c091ce3f79d031b3
Waiting for transaction Confirmation...
Transaction confirmed!
Stored Value in Contract: 100
```

## 📌 Notes
- Always keep your private keys secure and never expose them in public repositories.
- Use a testnet (like infura) for development instead of mainnet to avoid spending real ETH.

