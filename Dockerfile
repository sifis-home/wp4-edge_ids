###############################################################################
# The netspot_control image is built on multiple steps. First, we use the
# GO-BUILDER image to build the netspot binary and then the RUST-BUILDER to
# build the netspot_control binary. These binaries are then copied to the final
# image. The final image is based on debian:bullseye-slim for a smaller size.
###############################################################################

###############################################################################
# Building the netspot

FROM golang:bullseye as GO-BUILDER

# Install development packages
ARG PACKAGES="libpcap0.8-dev"
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt upgrade -y && apt install -y $PACKAGES

# Cloning netspot
WORKDIR /build
ARG VERSION=2.1.2
RUN git clone -b v${VERSION} https://github.com/asiffer/netspot.git netspot

# Building
WORKDIR /build/netspot
RUN make -j$(nproc)
RUN make install_bin

# Building the netspot
###############################################################################

###############################################################################
# Building the netspot_control

FROM rust:bullseye as RUST-BUILDER

# Install development packages
ARG PACKAGES="libpcap0.8-dev"
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt upgrade -y && apt install -y $PACKAGES

# Building netspot control
WORKDIR /opt/netspot_control
COPY . .
RUN cargo build --release

# Building the netspot_control
###############################################################################

###############################################################################
# Making the final image

FROM debian:bullseye-slim

# Install required packages
ARG PACKAGES="libpcap0.8 libsqlite3-0"
RUN DEBIAN_FRONTEND=noninteractive apt update && DEBIAN_FRONTEND=noninteractive apt upgrade -y && DEBIAN_FRONTEND=noninteractive apt install -y $PACKAGES && rm -rf /var/lib/apt/lists/*
COPY --from=GO-BUILDER /usr/bin/netspot /usr/bin/netspot
COPY --from=RUST-BUILDER /opt/netspot_control/target/release/netspot_control /usr/bin/netspot_control
COPY static /opt/netspot_control/static

# Default Rocket server setting
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80

# Where to store cache and configuration database
ENV DB_FILE_PATH=/var/lib/netspot_control/netspot_control.db

# Start server
CMD ["netspot_control"]

# Making the final image
###############################################################################
