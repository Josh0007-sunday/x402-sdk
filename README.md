# x402 Protocol: Bitcoin Lightning Payments for HTTP APIs

## Overview

x402 is an open payment standard that enables services to charge for access to their APIs and content directly over HTTP. Built around the HTTP 402 "Payment Required" status code, x402 creates a seamless payment layer for both Web3 dApps and traditional web applications using Bitcoin's Lightning Network.

## How x402 Works with Lightning Network

The x402 protocol facilitates micropayments for digital content and API access through a simple, standardized flow:

### Payment Flow

1. **Content Request** - Client attempts to access paid content/API
2. **Invoice Creation** - Server generates a BOLT11 Lightning invoice
3. **402 Response** - Server returns HTTP 402 status with payment details
4. **Payment Execution** - User pays via WalletConnect or directly through their node
5. **Payment Verification** - System confirms settlement on Lightning Network
6. **Content Delivery** - Upon confirmation, server returns HTTP 200 with requested content

### Key Components

- **HTTP 402 Status Code**: Standard "Payment Required" response
- **BOLT11 Invoices**: Lightning Network payment requests
- **Wallet Integration**: Support for WalletConnect and direct node payments
- **Settlement Verification**: Automated payment confirmation
- **Seamless UX**: Minimal disruption to user experience

## Benefits

- **Universal Compatibility**: Works with both Web3 decentralized applications and traditional web apps
- **Bitcoin Native**: Leverages Bitcoin's security and Lightning Network's speed
- **Microtransactions**: Enables pay-per-use API access and content monetization
- **Standardized Protocol**: Built on existing HTTP standards
- **User Control**: Users maintain custody of funds throughout the process

## Use Cases

- API monetization
- Premium content access
- Pay-per-download services
- Micro-donations
- Subscription services with granular pricing

x402 represents a significant step toward making Bitcoin payments a native feature of the web, providing a simple infrastructure that bridges the gap between traditional web applications and decentralized finance.

