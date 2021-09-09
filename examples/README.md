# Example Instructions

Gathered here are some additional instructions on how to build and run the examples. Note that the examples are usually kept up-to-date with the most recent version of the code. If you are looking for example code compatible with a version published on crates.io, take a look at the [release tags](https://github.com/gschup/ggrs/tags).

## BoxGame

BoxGame is a very basic 2-4 player game example with each player controlling a coloured box.
There is no real game, just movement with ice physics. Optionally,
you can specify spectators.

- W to accelerate forwards
- S to accelerate backwards
- A to turn left
- D to turn right

### Important Disclaimer - Determinism

Since BoxGame is based on floats and uses floating-point sin, cos and sqrt,
I fully expect this example to desync when compiled on two different architectures/platforms.
This is intentional to see when and how that happens. If you plan to implement your own
deterministic game, make sure to take floating-point impresicions and non-deterministic results into consideration.

### Launching BoxGame P2P and Spectator

The P2P example is launched by command-line arguments:

- `--local-port / -l`: local port the client is listening to
- `--players / -p`: a list of player addresses, with the local player being identified by `localhost`
- `--spectators / -s`: a list of spectator addresses. This client will act as a host for these spectators

For the spectator, the following command-line arguments exist:

- `--local-port / -l`: local port the client is listening to
- `--num-players / -n`: number of players that will participate in the game
- `--host / -h`: address of the host

For example, to run a two-player game and a spectator on your local machine,
run these commands in separate terminals:

```shell
cargo run --example box_game_p2p -- --local-port 7000 --players localhost [::1]:7001 --spectators [::1]:7002
cargo run --example box_game_p2p -- --local-port 7001 --players [::1]:7000 localhost
cargo run --example box_game_spectator -- --local-port 7002 --num-players 2 --host [::]:7000 
```

## BoxGame SyncTest

The same game, but without network functionality.
Instead, the SyncTestSession focusses on simulating rollbacks and comparing checksums.
You can use the Arrow Keys in addition to WASD in order to move the second player.

### Launching BoxGame SyncTest

BoxGame SyncTest is launched by a single command-line argument:

- `--num-players / -n`: number of players that will participate in the game
- `--check-distance / -c`: number of frames that will be rolled back and resimulated each frame

```shell
cargo run --example box_game_synctest -- --num-players 2 --check-distance 7
```

## Rapier SyncTest

A stress test with the deterministic physics engine [Rapier](https://rapier.rs/).
You should run this with `--release` to ensure the code will be optimized.

### Launching Rapier SyncTest

The SyncTest is launched by a single command-line argument:

- `--num-bodies / -n`: this number^2 many rigid bodies will be spawned
- `--check-distance / -c`: number of frames that will be rolled back and resimulated each frame

```shell
cargo run --release --example rapier_synctest -- --num-bodies 20 --check-distance 7
```
