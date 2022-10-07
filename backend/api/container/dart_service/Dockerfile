FROM rust:1.64 as builder

WORKDIR /app
COPY . .
RUN cargo build --release -p dart_service


FROM dart:latest

WORKDIR /app
COPY --from=builder /app/target/release/dart_service .
CMD [ "./dart_service" ]
