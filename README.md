## Railway Deployment Guide

### Environment Setup
1. Make sure your Railway project is connected to this Git repository
2. Railway will automatically detect this is a Rust project

### Required Environment Variables
No special environment variables are required. Railway will automatically set the `PORT` variable.

### Deployment Process
1. Push your code to Git
2. Railway will automatically build and deploy

### API Endpoints
- `GET /` - Health check
- `GET /health` - Health check
- `POST /keypair` - Generate Solana keypair
- `POST /token/create` - Create mint initialization instruction
- `POST /token/mint` - Create mint tokens instruction
- `POST /message/sign` - Sign message with private key
- `POST /message/verify` - Verify message signature  
- `POST /send/sol` - Create SOL transfer instruction
- `POST /send/token` - Create SPL token transfer instruction

### Testing Locally
```bash
cargo run
```

### Building for Production
```bash
cargo build --release
```
