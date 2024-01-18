# Bevy Snake

Snake clone written in bevy with powerups. With wasm support.

Note: recommend turning off the sound on the extreme difficulty :)

Controls: wasd + arrow keys. Controller arrow keys supported only during gameplay.

Food:
    Yellow - Reduce snake length by 3.
    Blue - Game slowdown.
    Green - Spawn 4 more food in the world.
    Red - Regular food.


# How to build

## Native
1. Install rust and cargo.
2. ```cargo build```

## Wasm
1. Install trunk.
2. ```trunk build```