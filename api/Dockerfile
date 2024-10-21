# Build the binary file
FROM rust:1.82.0-bullseye AS builder

WORKDIR /api
COPY . .

RUN cargo build --release

# Run the binary file
FROM debian:bullseye-slim

WORKDIR /api
COPY --from=builder /api/target/release/api ./api

RUN apt update && apt install -y ca-certificates

EXPOSE 8000
CMD ["api"]