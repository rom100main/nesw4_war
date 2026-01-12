# NESW4: New Extreme Strategical Warfare 4

A two-player cellular automaton strategy game built with Rust and eframe (egui).

## Overview

NESW4 is a competitive game where two players fight to dominate the grid by strategically choosing rules. Each player controls cells that spread based on neighbor patterns.

## Game Rules

- **Grid**: A 64x64 toroidal (wrapping) grid
- **Cell States**:
  - Neutral (gray)
  - Player 1 (blue)
  - Player 2 (red)

- **Rule System**: Each rule defines a pattern of 4 neighbors (top, right, bottom, left). When a cell's neighbors match a player's rule, that cell becomes theirs.

## Game Flow

1. **Starting Rule Selection**: Both players take turns selecting their initial rules
2. **Rounds (repeat)**:
   - **Battle Phase**: The grid evolves for 128 iterations based on player rules
   - **Shop Phase**: The loser of the battle shops first, then the winner. Players earn money and can buy new rules or upgrade spawn probability

## Building and Running

```bash
cargo run --release
```
