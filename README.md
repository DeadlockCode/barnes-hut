# Barnes-Hut
This is the official repository for the code shown in [How to make HUGE N-Body Simulations (N=1,000,000+)](https://youtu.be/nZHjD3cI-EU)

This repository consists of three branches:
1. [The master branch](https://github.com/DeadlockCode/barnes-hut).
    
    This is the code shown in the video and is my (mostly) faithful implementation of the original algorithm as described in the Barnes-Hut paper.
2. [The improved branch](https://github.com/DeadlockCode/barnes-hut/tree/improved).
    
    This modifies the original algorithm by a) storing the nodes in a cache friendly order and b) allowing multiple bodies to inhabit the same leaf node.
3. [The parallel branch](https://github.com/DeadlockCode/barnes-hut/tree/parallel).
    
    This is a crude attempt at parallelizing the improved branch to show its potential.

## Guide
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone the repository
3. If you're **not** on Windows, follow [this](https://github.com/DeadlockCode/n-body/issues/1)
4. Checkout the desired branch
5. Open the folder in a terminal
6. Run 'cargo run --release'
7. Enjoy

## Controls
- Scroll to zoom
- Middle mouse button to grab view
- Right mouse button to spawn a body
- Space to pause/continue
- E to open a menu where you can enable the quadtree visualization
