# Blackjack CLI in Rust

A simple, interactive command-line Blackjack game written in Rust built to learn core Rust concepts.

---

## Description
- **Single‐deck play**: A fresh 52-card deck is shuffled at the start of each round.  
- **Initial deal**: Each player and the dealer receive two cards; one of the dealer’s cards remains hidden until player turns end.  
- **Player choices**:  
  - **Hit**: draw another card  
  - **Stand**: end your turn with your current total  
  - **Double**: double your bet, draw exactly one more card, then stand  
  - **Surrender**: forfeit half your bet and end your turn immediately  
- **Dealer rules**: After all players have finished, the dealer reveals the hole hand and automatically draws until reaching 17 or more.
- **Ace valuation**: Aces count as 1 or 11 automatically to maximize the hand’s value without busting.  
- **Busts & pushes**: Any hand over 21 busts and loses. Ties (“pushes”) return the player’s bet.  
- **Betting & payouts**: Players start with 50 chips; winning pays double the bet
- **Replayable rounds**: Chip balances carry over; play multiple rounds until you choose to quit.

## Project structure
```
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs       
│   ├── lib.rs
│   └── implementations
│       ├── action.rs  # Choosing an action
│       ├── deck.rs    # Card, Suit and Deck (shuffle) logic
│       ├── game.rs    # Round flow: deal, player turns, dealer turn, settling scores
│       ├── hand.rs    # Hand draw & total-value calculation
│       ├── mod.rs 
│       ├── player.rs  # Player state, betting & payouts
└── README.md
```

## How to run

```bash
git clone https://github.com/cezary-rasinski/Blackjack.git
cd Blackjack
cargo bulid
cargo run
```
## Additional info
Game is currently run with 2 preset players (Mark and Jack)

