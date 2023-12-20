FROM rust:latest

WORKDIR /usr/src/app

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli --vers 0.2.89
RUN cargo install wasm-server-runner

COPY . .

RUN cargo build --target wasm32-unknown-unknown

CMD ["cargo", "run", "--target", "wasm32-unknown-unknown"]
