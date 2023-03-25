FROM debian:latest

RUN apt-get update && apt-get install -y \
    build-essential \
    curl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["./target/release/rogger"]
