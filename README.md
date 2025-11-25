# ProofSpace - Trusted Content Attestation

**A decentralized content attestation platform powered by AWS Nitro Enclaves and Sui blockchain**

Built for Walrus Haulout Hackathon 2025

---

## Overview

**ProofSpace solves the fundamental crisis of digital trust in an era of AI generated content and deepfakes.**

In a world where any content can be synthesized, edited, or fabricated in seconds, establishing authenticity has become impossible through traditional means. News organizations, legal systems, content creators, and enterprises are losing billions to misinformation while struggling to prove the provenance of legitimate digital assets.

ProofSpace introduces a revolutionary approach by combining **hardware level security with blockchain immutability**. We leverage AWS Nitro Enclaves isolated compute environments with cryptographic attestation to create unforgeable proofs of content authenticity. These attestations are then immortalized as NFTs on the Sui blockchain, creating a permanent, verifiable chain of custody.

### Why This Matters

**The Problem**: Traditional content verification relies on centralized authorities that can be compromised, manipulated, or shut down. Digital signatures can be spoofed. Timestamps can be altered. Metadata can be stripped. Current solutions are fundamentally broken.

**Our Innovation**: ProofSpace is the first platform to unite three critical security layers:

1. **Hardware-Backed Trust**: Nitro Enclaves provide military grade isolation, no administrator, no cloud provider, not even AWS can access or tamper with the attestation process
2. **Cryptographic Certainty**: Every attestation includes verifiable signatures from the enclave's hardware secured keys
3. **Permanent Provenance**: Sui blockchain ensures attestations remain accessible and verifiable forever, without dependency on any central authority

This isn't just another blockchain project or security tool it's infrastructure for truth in the digital age. When a court needs to verify evidence, when journalists need to prove footage is authentic, when companies need to demonstrate compliance, ProofSpace provides irrefutable proof that stands up to the highest scrutiny.

### Real-World Impact

Content creators can prove original authorship. Legal professionals can establish chains of evidence. Enterprises can demonstrate regulatory compliance. News organizations can verify sources. AI companies can prove training data provenance. The applications are limitless, and the need is urgent.

---

## Features

**Core Capabilities**

- Cryptographic content attestation with hardware-backed security
- Secure hash computation in isolated AWS Nitro Enclave environment
- NFT minting on Sui blockchain for permanent proof storage
- Instant verification of previously attested content
- Comprehensive dashboard for proof management
- Seamless Sui wallet integration with multi wallet support

---

## Architecture

**Frontend** → React + TypeScript + Tailwind CSS  
**Backend** → AWS Nitro Enclave API (secure computation)  
**Blockchain** → Sui Network (NFT storage)  
**Smart Contracts** → Move language  
**Wallet Integration** → @mysten/wallet-kit

### How It Works
1. User submits content → Sent to Nitro Enclave
2. Enclave computes hash + cryptographic signature
3. Attestation data returned to user
4. User mints NFT → Transaction sent to Sui blockchain
5. Proof permanently stored on-chain

---

## Quick Start

### Prerequisites
- Node.js (v18+)
- npm or yarn
- Sui Wallet browser extension

### Installation

Clone and install dependencies:
```bash
git clone https://github.com/CaptainDiv/Proofspace.git
cd nautilus\frontend-final
npm install
```

### Run Development Server
```bash
npm run dev
```

App runs at **http://localhost:5173**

### Important: Chrome CORS Configuration

close all chrome windows first

Since the app connects to an external enclave API, you need to run Chrome with CORS disabled for development:

**Windows:**
```bash
chrome.exe --user-data-dir="C:/Chrome dev session" --disable-web-security
```

**macOS:**
```bash
open -na "Google Chrome" --args --disable-web-security --user-data-dir="/tmp/chrome-dev" http://localhost:5173
```

**Linux:**
```bash
google-chrome --disable-web-security --user-data-dir="/tmp/chrome-dev" http://localhost:5173
```

**Note:** This opens an isolated Chrome instance specifically for testing. Your regular Chrome browser remains secure and unaffected.

### Build for Production
```bash
npm run build
npm run preview
```

---

## Usage

### 1. Connect Wallet
Click "Connect Wallet" and approve connection in your Sui wallet

### 2. Attest Content
- Enter content in the text area
- Click "Get Attestation"
- View attestation data (hash, signature, timestamp)

### 3. Mint NFT Proof
- After receiving attestation, click "Mint Proof NFT on Sui"
- Approve transaction in wallet
- View NFT on Sui Explorer

### 4. Verify Content
- Navigate to "Verify" tab
- Paste content to verify
- See authenticity results instantly

---

## Technology Stack

**Frontend**: React 18, TypeScript, Vite, Tailwind CSS, Lucide Icons  
**Blockchain**: Sui, Move Smart Contracts, Sui SDK, Wallet Kit  
**Backend**: AWS Nitro Enclaves, Ed25519 Signatures, REST API  

**Smart Contract Details**:
- Package ID: `0x152fab2e400917a380f5fd22491e6ce6a773530372c16c4fe55a63a37cc9f094`
- Module: `content_attestor`
- Network: Sui Testnet

**Live Demonstration:**
  https://proofspacee-oyop38b6y-divineosaigbovo1-5376s-projects.vercel.app
---

## Hackathon Challenges Addressed

**Security Innovation**: Nitro Enclaves provide hardware level security guarantees that cannot be compromised by software vulnerabilities, malicious administrators, or cloud infrastructure breaches. This represents a paradigm shift from trust based to proof based security.

**Blockchain Scalability**: Sui's parallel execution and object centric model enables high throughput with sub second finality, making real time attestation practical at scale.

**True Decentralization**: Complete elimination of single points of failure. Attestations remain verifiable even if ProofSpace ceases operations, creating genuinely unstoppable infrastructure.

**User Experience Excellence**: Complex cryptographic operations abstracted into a three click process. Non-technical users can create unforgeable proofs as easily as posting on social media.

**Market Innovation**: First platform to combine Nitro Enclaves with blockchain for content attestation, opening entirely new markets for digital provenance and trust services.

---

## Security Features

- Isolated execution in AWS Nitro Enclaves
- No persistent storage in enclave
- Cryptographic signature verification
- Immutable on chain proof storage
- Hardware-backed attestation

---

## Future Roadmap

- Batch attestation processing
- IPFS integration for large files
- Mobile app (React Native)
- Enterprise API
- Zero-knowledge proof verification

---

## Project Structure

```
proofspace/
├── move/
|   ├── content_attestor
|   ├── enclave
├── src/
│   ├── App.tsx           # Main component
│   ├── main.tsx          # Entry point
│   └── index.css         # Styles
├── public/               # Static assets
├── package.json
├── vite.config.ts
└── README.md
``` 
---

## Acknowledgments

Built with dedication for Walrus Haulout Hackathon.

Special thanks to Sui Foundation, AWS, and the open-source community.

---
