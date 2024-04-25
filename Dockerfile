FROM rust:1.77.2-slim-buster as build

WORKDIR /app

RUN apt-get update && apt-get install -y musl-tools libpq-dev

COPY . .

RUN cargo build --release

FROM rust:1.77.2-slim-buster

WORKDIR /app

RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres

COPY --from=build /app/diesel.toml .
COPY --from=build /app/migrations ./migrations
COPY --from=build /app/src/schema.rs ./src/schema.rs

RUN diesel migration run

COPY --from=build /app/target/release/user-management .

CMD ["./user-management"]