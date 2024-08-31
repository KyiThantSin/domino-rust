# Mini Domino Text Game
A simple command-line Domino game implemented in Rust for a KMITL University first year first semester pre-midterm project.

## Features
- Game Setup: Randomized dominoes dealt to player and computer.
- Gameplay: Turn-based play with color-matching rules.
- Victory Conditions: Announced when a player or computer wins.

## Game Concept
The Domino Game is a classic tile-matching game where players take turns placing dominoes with matching colors on the board. The game starts with each player receiving a set of randomly shuffled dominoes. The player who successfully places all their dominoes or forces the opponent into an invalid move wins the game.

## How to Play
- Objective: Match the color of the dominoes' ends with the starting domino on the board to play them.
- Starting the Game: The game begins with each player receiving a set of dominoes and a starting domino is placed by the computer.

### Player's Turn:
Choose a domino from your hand.
The chosen domino must match the color of the starting domino's end.
If you have no valid moves and the computer has valid moves, the computer wins.
If the move is valid, it updates the starting domino.
### Computer's Turn:
The computer will select a valid domino from its hand randomly and place it on the board.
If the computer has no valid moves and the player has valid moves, the player wins.

### Win/Loss Conditions
Win:
The player wins if they place all their dominoes on the board before the computer does.
The computer wins if the player cannot make a valid move while the computer can.
Loss:
The game ends with the player losing if the computer places all its dominoes first or if the player has no valid moves while the computer still can.
