# Build Stage
FROM rust:1.74-bullseye as builder
LABEL authors="Henrik Hausberg & Anders Morille"

# Create a new empty shell project
RUN USER=root cargo new --bin rust-server
WORKDIR /rust-server

# Copy the manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependencies are built, copy your source code
COPY ./src ./src

# Build your application
RUN rm ./target/release/deps/rust_server*
RUN cargo build --release

# Final Stage
FROM debian:buster-slim

# Copy the build artifact from the build stage
COPY --from=builder /rust-server/target/release/rust-server .

# Set the startup command to run your binary
CMD ["./rust-server"]
