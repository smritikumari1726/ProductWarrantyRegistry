# ProductWarrantyRegistry
A Soroban smart contract on Stellar for secure, transparent, and tamper-proof product warranty management on-chain.
<img width="1920" height="1140" alt="Screenshot 2026-03-19 144758" src="https://github.com/user-attachments/assets/62f81aa3-bc8f-4d0c-98de-a5ecc3839a83" />

# Product Warranty Registry

## Project Description
Product Warranty Registry is a basic Soroban smart contract built on the Stellar network for storing and managing product warranty records on-chain.

The contract allows an admin to register warranties for products, store ownership details, and let users verify whether a warranty is still active and valid. This creates a transparent and tamper-resistant warranty system that can reduce fake claims and improve trust between manufacturers, sellers, and customers.

---

## What it does
This smart contract acts as an on-chain warranty database.

It allows:
- Initializing the contract with an admin
- Registering a new warranty for a product
- Retrieving warranty information by product ID
- Transferring warranty ownership to another wallet address
- Deactivating a warranty
- Checking whether a warranty is still valid based on time and status

Each warranty record stores:
- Product ID
- Serial number
- Owner wallet address
- Issuer wallet address
- Purchase date
- Warranty end date
- Metadata URI
- Active/inactive status

---

## Features
- **Admin-controlled initialization**
- **On-chain product warranty registration**
- **Warranty ownership transfer**
- **Warranty validity check using ledger timestamp**
- **Warranty deactivation support**
- **Transparent and immutable record keeping**
- **Built with Soroban SDK for Stellar**

---

## Deployed Smart Contract Link
🔗 https://stellar.expert/explorer/testnet/tx/4d49cf3bc3ad03eaa5449d699406dd6c79bdf257379e7d692ca7cab87462ec43
🔗 https://lab.stellar.org/r/testnet/contract/CDJPR2A6ZBOA2RMOQ6PC7WLA2FNXA3DDIIFDDYQFZAL5PX3RBJMDMFX5
