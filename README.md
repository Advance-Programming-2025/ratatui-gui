# One Million Crabs - The Game

> *Welcome Commander. You are tasked to lead the crab-driven expansion across the galaxy. Do you have what it takes?*

## Overview

Welcome to One Million Crabs (OMC)'s galaxy terminal visualizer! This repo houses the front-end of the 2025/26 Advanced 
Programming course project, built with RataTui. Explorers try to navigate the galaxy autonomously (or manually, if you 
think you can do better than our AI!) to avoid asteroids and generate as many resources as possible to generate even
more things. Who will survive the longest? Who will create the most resources? Who will make the most AI partners? 
Take it for a spin and find out!

## Run the project

The project is built in Rust and as such it can be run with Cargo. simply:

1. Clone the project
```bash
git clone https://github.com/Advance-Programming-2025/ratatui-gui.git
```
2. Build and run
```bash
cargo run
```


## Architecture
The architecture is modular and state‑driven, with a clear separation between the game engine (orchestrator), the application state (App), the controller (input‑to‑command mapping), and the view (rendering). This makes the codebase maintainable and extensible.

### Game Engine
The orchestrator intermediates the communication between the actors of the game (planet and explorer) and the UI with a dedicated layer of APIs. But you can find more details here.

### Application State
The App struct reflects what the state of the game is, by taking frequent snapshots of the orchestrator and making the information available to be shown on the terminal.

### Controller
The controller translates raw key events into semantic actions and decides which commands to emit based on the current game state, UI mode, and focus. It is stateless and returns a list of side‑effect commands that are then applied to the App and the orchestrator.

### View
The view is built with ratatui and organised into screens (start, game, paused, ended). It uses a centralised theme and reusable selectors to manage list/table highlights. Rendering is split across dedicated modules (planets, explorers, logs, instructions) and leverages view models to keep presentation logic separate from raw data.

### Advanced rust features
- Generic functions to reduce code duplication for shared logic (e.g., ResourceSelector<T>).

- Use of Arc and Mutex for safe sharing of the log buffer between threads.

---

Built by Marco Adami for the 2025/26 Advanced Programming course @ UniTN