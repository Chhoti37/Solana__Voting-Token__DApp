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
>Build the program 
  anchor build
>Get the Program ID 
  solana address -k target/deploy/vote_app-keypair.json
>Update the Program ID in anchor.tomal and program/vote_app/src/lib.rs
>Then deploy to Devent or localhost
  anchor deploy
>
3.Frontend Setup
cd client
npm install
npm run dev

📸 App Preview
![Solana Voting DApp Screenshot]
🖥️ 1. Interactive User Dashboard
Description: Is screen par user apna wallet balance dekh sakta hai aur voting status check kar sakta hai. Ye hamare DApp ka main entry point hai.
![Screenshot_9-4-2026_04421_localhost](https://github.com/user-attachments/assets/a2ddab6c-afa8-4887-b578-cded74999897)

🛡️ 2. Admin Management
Description:The application features a secure admin portal to manage the voting ecosystem.
![Screenshot_9-4-2026_04522_localhost](https://github.com/user-attachments/assets/e4cdce49-c5c6-4a9e-a43c-0d1d7a7ff1c4)

🗳️ 3. All Active Proposals
Description: Isme saari active voting proposals ki list dikhti hai, jahan user apna vote cast kar sakta hai.
![Screenshot_9-4-2026_04722_localhost](https://github.com/user-attachments/assets/ad649f11-2d81-4792-9bc7-8c4855ab2580)

💡 Key Learnings
Architecting Solana programs using the Anchor framework.
Implementing PDAs for unique, deterministic account derivation.
Handling Cross-Program Invocations (CPI) and token transfers.
Connecting a modern React frontend to the blockchain using IDL.

👩‍💻 Author
Developed with ❤️ by CHHOTI KUMARI
B.Tech Student at Narula Institute of Technology

⭐ If you like this project, please give it a Star!
