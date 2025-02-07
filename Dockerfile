# STAGE1: Build the binary
FROM rust:1.84.1 as builder

# Create a new empty shell project
WORKDIR /app

# Copy over the Cargo.toml files to the shell project
COPY ./ ./

RUN cargo build --release

# STAGE2: create a slim image with the compiled binary
FROM alpine as runner

# Copy the binary from the builder stage
WORKDIR /app
COPY --from=builder /app/target/release/app app

CMD ["./app", "loopapp"]
