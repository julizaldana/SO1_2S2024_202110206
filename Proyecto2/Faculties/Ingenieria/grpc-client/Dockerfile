# Build stage: compilación del binario
FROM rust:1.72 as builder

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Runtime stage: con versión de glibc más actualizada
FROM ubuntu:22.04 

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/grpc-client .

CMD ["./grpc-client"]
