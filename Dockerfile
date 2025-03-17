# STAGE1: Build the binary
FROM rust:1.85.0 as builder

# Create a new empty shell project
WORKDIR /app

# Copy over the Cargo.toml files to the shell project
COPY ./ ./

RUN cargo build --release

# STAGE2: create a slim image with the compiled binary
FROM rust:1.85.0

# Copy the binary from the builder stage
WORKDIR /app
COPY --from=builder /app/target/release/app app

CMD ["./app", "loop"]
