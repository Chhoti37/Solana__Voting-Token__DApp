# 🗳️ Solana Voting Token DApp

A decentralized voting application built on the **Solana Blockchain** using the **Anchor Framework**. This project allows users to participate in governance by voting on proposals using custom tokens and Program Derived Addresses (PDAs).

## 🚀 Features
* **On-Chain Voting:** Secure and transparent voting logic developed with Rust.
* **Smart Contract:** Built using the Anchor framework for better security and structure.
* **Frontend:** Interactive User Interface built with React.js & Vite.
* **Wallet Integration:** Seamless connection with Phantom and other Solana-compatible wallets.
* **State Management:** Efficient use of PDAs for storing proposal and vote data.

## 🛠️ Tech Stack
* **Blockchain:** Solana
* **Framework:** Anchor (Rust)
* **Frontend:** React.js, Tailwind CSS
* **Library:** @solana/web3.js, @coral-xyz/anchor
* **Environment:** Node.js, Solana CLI

## 📂 Project Structure.
├── programs/vote_app   # Solana Smart Contract (Rust/Anchor)
├── client/             # React Frontend Application
├── Anchor.toml         # Anchor Configuration File
├── package.json        # Root Dependencies
└── README.md           # Project Documentation

⚙️ How to Run Locally
1. Prerequisites
> Install Solana Tool Suite
> Install Anchor Framework
> Install Node.js (v18 or above)

2. Smart Contract Setup
# Build the program
anchor build
# Get the Program ID
solana address -k target/deploy/vote_app-keypair.json
# Update the Program ID in Anchor.toml and programs/vote_app/src/lib.rs
# Then deploy to Devnet
anchor deploy

3.Frontend Setup
cd client
npm install
npm run dev

💡 Key Learnings
Architecting Solana programs using the Anchor framework.
Implementing PDAs for unique, deterministic account derivation.
Handling Cross-Program Invocations (CPI) and token transfers.
Connecting a modern React frontend to the blockchain using IDL.

👩‍💻 Author
Chhoti Kumari
B.Tech Student at Narula Institute of Technology
