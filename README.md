# Critter Cove: Zoo Manager

Critter Cove: Zoo Manage is a Zoo Tycoon like game written in rust using the bevy framework.

## Developing The Game

### Running The Game

> cd ~/Critter-Cove-Zoo-Manager
>
> cargo build
>
> cargo run

Or using bacon

> cd ~/Critter-Cove-Zoo-Manager
>
> bacon
>
> r

### Testing The Game

> cd ~/Critter-Cove-Zoo-Manager
>
> cargo test

Or using bacon

> cd ~/Critter-Cove-Zoo-Manager
>
> bacon
>
> t

### Linting The Game

> cd ~/Critter-Cove-Zoo-Manager
>
> cargo clippy --fix --allow-dirty

Or using bacon

> cd ~/Critter-Cove-Zoo-Manager
>
> bacon
>
> c

### Formatting The Game

> cd ~/Critter-Cove-Zoo-Manager
>
> cargo fmt

Or using bacon

> cd ~/Critter-Cove-Zoo-Manager
>
> bacon
>
> f

## Dependencies

Follow the steps for installing rustc runtime for your given operating system.

> <https://www.rust-lang.org/tools/install>

Install bacon

> cargo install bacon

Install the dependency packages

> bash setup_dependencies.sh

Please use good op-sec habits and check the contents of both .sh files before running them as these will modify your system
