
# Compilation

## Compiler au fur et à mesure des changements :

```shell
# Current html is release
# release mode (remove the `--release` flag for debug mode)

cargo watch -- cargo build --target wasm32-unknown-unknown --bin platformer_graphique

cargo watch -- cargo build --release --target wasm32-unknown-unknown --bin platformer_graphique

# cargo watch --ignore '**/generated/**' -- cargo build --release --target wasm32-unknown-unknown --bin platformer_graphique

# base command
cargo watch -- cargo run --package=platformer_graphique
```

+ Lancer le web serveur / extension VS Code : Live Server

## Compiler en une seule fois

```shell
cargo build --target wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown

cargo build --bin=platformer_graphique --package=platformer_graphique --release --target wasm32-unknown-unknown

cargo run --package=platformer_graphique --release
```

## Create a new project in the workspace

```shell
cargo new NAME --lib
```

```shell
cargo run --package=platformer_graphique
```


### Pc

```shell
cargo run
cargo run --release
```



### Cool other commands :

benchmarks : google it
```shell
cargo bench

#[bench]
```

### Android

Todo : Don't work

<https://macroquad.rs/articles/android/>


Démarrer Docker sur windows (Docker Desktop)


docker pull notfl3/cargo-apk

docker run --rm -v ${PWD}:/root/src -w /root/src notfl3/cargo-apk cargo quad-apk build --release --package=platformer_graphique


// update rust inside docker
docker run --rm -v ${PWD}:/root/src -w /root/src -it notfl3/cargo-apk /bin/bash

docker pull rust

rustup update stable

cargo quad-apk build --release --package=platformer_graphique


rustup override set 1.73.0
rustup override set stable


rustc --version


### Android Old

Dont work for the moment
cargo apk build --example rustaceanmark


<https://macroquad.rs/articles/android/>

Démarer Docker sur windows (Docker Desktop)

```
docker pull notfl3/cargo-apk



docker run -it --rm -v ${PWD}:/root/src -w /root/src/fronts notfl3/cargo-apk /bin/bash

rustup update stable

rustup default stable

rustup target add armv7-linux-androideabi

cargo quad-apk build --release --package=front


docker run --rm -v ${PWD}:/root/src -w /root/src/fronts notfl3/cargo-apk cargo quad-apk build --release --package=front

docker run rustup update
```


#### Other

```shell
# Generate the documentation :
cargo doc

# Find more complexe warnings
cargo clippy
```

Also see `programming_convention.md`

# Author
- Thomas Mewily 2024