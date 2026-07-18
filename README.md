# Ferris Jumping Game

[![Rust](https://img.shields.io/badge/Rust-1.85%2B-orange?logo=rust)](#)
[![Macroquad](https://img.shields.io/badge/Macroquad-0.4.15-blue?logo=rust)](#)
[![Rapier2D](https://img.shields.io/badge/Rapier2D-0.30.1-success)](#)
[![GitHub Pages](https://img.shields.io/badge/GitHub_Pages-Live-brightgreen?logo=github)](https://yakisobites.github.io/jumping_game/)

🚀 **[Play Now in Your Browser!](https://yakisobites.github.io/jumping_game/)**

---

## 🎮 Overview

**Ferris Jumping Game** is a simple yet playful game project built with Rust, leveraging the **Rapier2D** physics engine and **Macroquad** for lightweight rendering.

Control Ferris, balance through snappy physics-driven mechanics, keep bouncing, and aim for the high score! This project is designed for anyone wanting to explore Rust game development, balancing clean code architecture with pure playability.

![demo](assets/demo.png)

## ✨ Features

- **Real-time Physics**: Powered by Rapier2D to simulate satisfying inertia, gravity, and bounciness.
- **Lightweight Rendering & Input**: Utilizing Macroquad for quick, smooth rendering and snappy controls.
- **Score Attack Gameplay**: Keep Ferris alive and balanced to rack up the survival time.
- **Complete Game Loop**: Structured with a title screen, active gameplay state, and a game-over sequence.
- **Hackable Base**: A clean starting point perfect for adding obstacles, juicy visual effects, or expanding stages.

## 📦 Dependencies

- [rapier2d](https://github.com/dimforge/rapier) - 2D Physics Engine for Rust
- [macroquad](https://github.com/not-fl3/macroquad) - Lightweight Game Rendering & Input Library

## 🦀 Credit

The `ferris.png` asset is sourced from the [macroquad](https://github.com/not-fl3/macroquad) repository.

## 🕹️ Controls

- `Enter`: Start Game (Title Screen)
- `↑`: Thrust / Jump
- `←`: Move Left
- `→`: Move Right
- `R`: Restart (Game Over Screen)
- `ESC`: Return to Title / Exit

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2024 / 1.85+)
- [just](https://github.com/casey/just) — command runner (`cargo install just`)
- **Either** [Docker](https://docs.docker.com/get-docker/) *(recommended, no extra setup needed)*  
  **or** the local toolchain below

#### Local toolchain (without Docker)

```sh
# 1. Install binaryen (provides wasm-opt)
sudo apt update && sudo apt install -y binaryen   # Ubuntu/Debian
# brew install binaryen                           # macOS

# 2. Install basic-http-server
cargo install basic-http-server

# 3. Add the WASM target
rustup target add wasm32-unknown-unknown
```

### Run locally

```sh
# Automatically uses Docker if available, otherwise falls back to local toolchain
just run
```

The game will be served at **http://localhost:4000**.

#### Individual steps

| Command | Description |
|---|---|
| `just build` | Compile to WebAssembly |
| `just dist` | Optimize WASM and copy assets to `dist/` |
| `just serve` | Start HTTP server on `dist/` |
| `just docker-image` | Build the Docker image |
| `just docker-run` | Build & serve inside Docker |
| `just test` | Run fmt / clippy / check / tests |

## 🛠️ Development Notes

- Built using the **Rust 2024 Edition**.
- Combining Rapier2D and Macroquad makes this a fantastic micro-project for both learning Rust and rapid prototyping.
