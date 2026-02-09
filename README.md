# CertiChain
CertiChain is a Solana program (simulated in Playground) to issue and verify student certificates.

## Problem
Fake certificates are easy to create and hard to verify.  
Colleges and employers waste time manually checking credentials, and students face fraud risks.

## Solution
Certificates are stored on Solana blockchain (simulated here).  
Each certificate is uniquely tied to the student and the issuing wallet.  
Anyone can verify a certificate using the certificate account public key.

## Tech Stack
- Solana
- Anchor (Rust)
- Solana Playground
- TypeScript (`client.ts` for wallet & balance demo)

## How It Works
1. College connects wallet (identity)
2. Fills certificate info (student_name, course, college) in Playground Client tab
3. Certificate is stored on-chain (or simulated)
4. Anyone can verify certificate using the certificate account public key

## Demo Instructions
1. Open the project in Solana Playground
2. Go to **Client tab**
3. Fill the form fields:
   - student_name: Rishav Shrestha
   - course: Blockchain Basics
   - college: Himalaya Darshan College
4. Click **Run**
5. Console prints the certificate account public key
6. Repeat to issue multiple certificates → each will have a **unique account**

---

## Evaluation Criteria Covered
|--------------------------|------------------------------------------------------------------------------|
| Criteria                 | How CertiChain Meets It                                                      |
|--------------------------|------------------------------------------------------------------------------|
| Potential Impact         | Prevents fake certificates; builds trust; ready for real deployment          |
| Business Case            | Target users: colleges, training institutes; scalable; optional paid service |
| UX                       | 3-step certificate issuance; clear console feedback; simple workflow         |
|                          |                                                                              |
| Technical Implementation | Anchor/Rust smart contract; each certificate in a unique account;            |
| & Functionality          | Playground simulation proves end-to-end operation                            |
|--------------------------|------------------------------------------------------------------------------|

## Notes
- Playground simulates Solana accounts, so **no SOL is required**.  
- Wallet identity represents the issuing college.  
- Each certificate account is unique even if the wallet is the same.  

## Folder Structure
certichain-solana/
├── programs/
│   └── certichain/
│       └── src/
│           └── lib.rs
├── client.ts
└── README.md