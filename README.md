# wretris
w(asm) + r(ust) + (t)etris
<img width="982" alt="Screenshot 2023-07-26 at 4 43 56 PM" src="https://github.com/genieCS/wretris/assets/35099832/064c89e8-f401-4374-b0c6-9e2b4cc8d25e">

This is a simple implementation of the classic tetris game in Rust using [Cursive](https://github.com/gyscos/cursive) library.

demo: https://geniecs.github.io

# Features
This is a Tetris game implementation with additional keyboard shortcuts for easier block manipulation. In this game, you can move the blocks to the leftmost or rightmost position using the added shortcuts, and rotate them in the opposite direction and flipturn which is 180 degree rotation. This makes it easier to play the game with fewer keyboard inputs.

# Installation
To install and run the game, you'll need to have Rust and Cargo installed on your system. Once you have Rust and Cargo installed, you can clone the repository and run the game using the following commands:

```
git clone https://github.com/geniecs/wretris.git
wasm-pack build
cd www
npm install
npm run dev
```

# How to Play
The goal of the game is to clear as many lines as possible by fitting the falling blocks together. Use the keyboard controls to move and rotate the blocks as they fall. The game ends when the blocks reach the top of the screen or 40 lines are cleared.

# Controls
* a: Move the block to the leftmost position
* d: Move the block to the rightmost position
* w: Rotate the block counter-clockwise
* ↑ or e: Rotate the block clockwise
* ↓: Speed up the block
* space: Hard drop the block
* m: Stop and resume the game
* n: Start a new game

# Acknowledgements
This project was inspired by the classic Tetris game and Cursive library for Rust.

# License
This project is licensed under the MIT License. See the LICENSE file for details.
