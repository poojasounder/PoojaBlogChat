FROM rust:latest

WORKDIR /app/

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

<<<<<<< HEAD
CMD ["cargo", "watch", "--why", "-x", "build"]

=======
CMD ["cargo", "watch", "--why", "-x", "build"]
>>>>>>> 7b19c02a95b624f4b82302640aefb65fd9432bfa
