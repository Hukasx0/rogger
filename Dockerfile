FROM rust:latest

RUN apt-get update && apt-get install -y \
    apt-get install -y sqlite3 libsqlite3-dev && \
    apt-get install -y redis-server \
    build-essential \
    curl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . .
COPY web/ web/

RUN cargo build --release

CMD ["sqlite3"]
CMD ["redis-server", "--daemonize", "yes"]
CMD ["./target/release/rogger"]
