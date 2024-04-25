FROM rust:1.77.2-slim-buster as build

WORKDIR /app

RUN apt-get update && apt-get install -y musl-tools libpq-dev

COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libpq-dev

COPY --from=build /app/target/release/user-management .

CMD ["./user-management"]