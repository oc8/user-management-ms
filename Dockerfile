FROM rust:1.77.2-slim-buster as build

WORKDIR /app

RUN apt-get update && apt-get install -y musl-tools libpq-dev libssl-dev pkg-config

COPY . .

RUN cargo build --release

FROM rust:1.77.2-slim-buster

WORKDIR /app

RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --version 2.1.1 --no-default-features --features postgres

COPY --from=build /app/migrations ./migrations
COPY --from=build /app/Makefile ./Makefile

COPY --from=build /app/target/release/user-management .

COPY --from=build /app/deployments/scripts/entrypoint.sh ./entrypoint.sh
RUN chmod +x entrypoint.sh

ENTRYPOINT ["./entrypoint.sh"]
