FROM rust:1.58 as builder

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]
