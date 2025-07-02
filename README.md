# Agent Trade Platform

A cryptocurrency trading platform with AI-powered signal detection and automated trading capabilities.

## Key Features
- Real-time trading signals using AI analysis
- Web-based dashboard for monitoring and configuration
- Telegram bot for notifications and trading
- Support for multiple blockchain networks (Ethereum, Solana)
- Automated trading execution
- Token analytics and market insights

## Project Structure
```
├── app/
│   ├── telegram-bot/    # Telegram notification and trading bot
│   └── web/             # Next.js web application (dashboard)
└── server/              # Rust backend server
    ├── src/
    │   ├── contracts/   # Blockchain smart contract interactions
    │   ├── service/     # Core services (data processing, AI signals)
    │   └── tools/       # Trading tools and utilities
    ├── configs/         # Blockchain configuration files
    └── tests/           # Contract tests
```

## Getting Started

### Prerequisites
- Node.js v18+ (for web app)
- Rust 1.70+ (for backend server)
- Telegram bot token (for notifications)
- Access to blockchain RPC endpoints

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/agent-trade.git
   cd agent-trade
   ```
2. Install web dependencies:
   ```bash
   cd app/web
   npm install
   ```
3. Build the Rust server:
   ```bash
   cd server
   cargo build --release
   ```

### Running the Application
1. Start the backend server:
   ```bash
   cd server
   cargo run --release
   ```
2. Start the web application:
   ```bash
   cd app/web
   npm run dev
   ```
3. Access the web dashboard at: `http://localhost:3000`

## Configuration
Edit `server/config.json` to configure:
- Blockchain RPC endpoints
- Trading strategies and parameters
- Exchange API keys
- Telegram bot credentials
- AI model settings

## Support
For issues or feature requests, please open an issue in our GitHub repository.
