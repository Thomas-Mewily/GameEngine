This game engine is not finished yet !
Under construction !

# Compilation

## Compile as changes are made : *au fur et à mesure*

## Web Assembly

```shell
# The current html is looking for the debug wasm
# Remove the `--release` flag for debug mode

# debug
cargo watch -- cargo build --target wasm32-unknown-unknown --bin platformer_graphique

# release
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