Todo : Don't work

Tentatives :
### Android

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
