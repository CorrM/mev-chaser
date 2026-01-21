# MevChaser

> **Disclaimer**: This software is for **educational purposes only**. MEV (Maximal Extractable Value) strategies are highly competitive and risky. Use this at your own risk. The authors are not responsible for any financial losses.

**MevChaser** is a high-performance MEV bot written in Rust, designed to capture arbitrage opportunities on EVM-compatible chains (Ethereum, Polygon) using **backrunning** strategies.

It leverages Uniswap V2-compatible DEXes, Balancer flash loans, and advanced simulation techniques to identify and execute profitable trades in real-time.

## 🚀 Features

*   **Backrunning Strategy**: Monitors the mempool for pending transactions and attempts to backrun them for profit.
*   **Arbitrage Logic**: Supports multi-hop arbitrage paths and triangular arbitrage.
*   **Flash Loans**: Integrates with **Balancer** to borrow funds for arbitrage execution without capital requirements.
*   **Uniswap V2 Support**: Compatible with any Uniswap V2 fork (QuickSwap, SushiSwap, etc.).
*   **Real-time Simulation**: Uses `debug_trace_call` to simulate transactions and calculate precise state changes before execution.
*   **Bundling & Relaying**: Supports private transaction submission via relays (Flashbots, Marlin, FastLane) to avoid frontrunning.
*   **Optimized Performance**: Built with Rust + Rayon for parallel processing of opportunities.

## 🏗 Architecture

The project is organized as a Rust workspace with the following structure:

*   **`apps/cli`**: The main entry point and runner for the bot.
*   **`packages/amm`**: Logic for Automated Market Makers (Uniswap V2).
*   **`packages/contracts`**: Rust bindings for the smart contracts.
*   **`packages/mev`**: Core MEV logic, including strategies (`BackRunnerStrategy`), bundling, and relay integration.
*   **`packages/shared`**: Shared utilities, networking, and token management.
*   **`contracts`**: Solidity smart contracts used for arbitrage execution (`V2ArbBot`) and flash loans.

## 🛠 Prerequisites

*   **Rust**: [Install Rust](https://www.rust-lang.org/tools/install).
*   **Foundry**: Required for compiling smart contracts. [Install Foundry](https://book.getfoundry.sh/getting-started/installation).
*   **SQLite**: The bot uses a local `Main.db` SQLite database to store token and pool information.
*   **RPC Node**: You need access to a fast RPC node (WebSocket & HTTP) that supports `debug_trace_call` (e.g., Alchemy, Blockpi, or a local Geth/Erigon node).

## ⚙️ Setup

### 1. Clone the Repository

```bash
git clone https://github.com/CorrM/mev-chaser.git
cd mev-chaser
```

### 2. Compile Smart Contracts

Go to the contracts directory and build the Solidity artifacts:

```bash
cd tmp/contracts
forge build
cd ../..
```

### 3. Environment Configuration

Create a `.env` file in the root directory (or `apps/cli`) with the following variables:

```env
# Network Configuration
CHAIN_ID=137                # e.g., 137 for Polygon, 1 for Ethereum
HTTPS_URL=https://...       # Your HTTP RPC URL
WSS_URL=wss://...           # Your WebSocket RPC URL
BLOCKPI_API_KEY=...         # Optional: If using Blockpi

# Bot Configuration
PRIVATE_KEY=0x...           # Private key of the bot wallet
SIGNING_KEY=0x...           # Signing key (for Flashbots/Relays)
BOT_ADDRESS=0x...           # Public address of the bot wallet
```

### 4. Database Initialization

The bot relies on a SQLite database (`Main.db`) to track tokens and liquidity pools.

1.  **Initialize the Database**: Ensure `Main.db` exists (the application expects it).
2.  **Add Tokens**: Populate the database with tokens to monitor. You can use the provided `add_tokens.txt` list.

    ```bash
    cargo run --bin cli -- add_token add_tokens.txt
    ```

3.  **Generate Pools**: Scan the factory for pairs involving the added tokens.

    ```bash
    cargo run --bin cli -- gen_pools
    ```

## 🏃 Usage

To start the bot in production mode:

```bash
cargo run --release --bin cli
```

### How it Works

1.  **Monitoring**: The bot subscribes to the `newPendingTransactions` stream from the RPC node.
2.  **Filtering**: It filters transactions interacting with known Uniswap V2 router addresses.
3.  **Simulation**: It simulates the target transaction using `debug_trace_call` to see which pools are touched and how reserves change.
4.  **Optimization**: It calculates the optimal arbitrage path (if any) that results from the state change.
5.  **Execution**: If a profitable path is found (profit > gas cost), it constructs a backrun transaction (using flash loans if necessary) and submits it via a private relay or directly to the network.

## 🧪 Testing

To run the Rust tests:

```bash
cargo test
```

To test the smart contracts via Foundry:

```bash
cd contracts
forge test -vv
```

## 📜 License

See the [LICENSE](LICENSE) file for details.
