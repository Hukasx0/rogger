FROM rust:latest

RUN apt-get update && apt-get install -y \
    libsqlite3-dev \
    build-essential \
    curl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . .
COPY templates/ templates/

RUN cargo build --release

CMD ["./target/release/rogger"]
