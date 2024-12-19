FROM rust:latest AS builder

WORKDIR /rust

COPY ./ .

RUN cargo build --release



FROM registry.access.redhat.com/ubi9-minimal

WORKDIR /rust

# Copy our build
COPY --from=builder /rust/target/release/rust-echo-server ./

EXPOSE 3000

ENTRYPOINT ["/rust/rust-echo-server"]
