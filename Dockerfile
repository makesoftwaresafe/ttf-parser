FROM rustlang/rust:nightly-bullseye

RUN cargo install -f cargo-fuzz

COPY . .

RUN cargo fuzz build font-info
