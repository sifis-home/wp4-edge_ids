# Building netspot_control with Rust container based on Debian
FROM rust:bullseye as rust-builder
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y libpcap0.8-dev
WORKDIR /app
COPY netspot_control .
RUN cargo install --path .

# Running server on smaller Debian container
FROM debian:bullseye-slim as server
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y libpcap0.8 && rm -rf /var/lib/apt/lists/*
COPY --from=rust-builder /usr/local/cargo/bin/netspot_control /usr/local/bin/netspot_control
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["netspot_control"]
