FROM rust:1.79 as builder

WORKDIR /app

COPY . /app

RUN apt-get update
RUN apt-get install protobuf-compiler -y

RUN cargo build --release

FROM rust:1.79

COPY --from=builder \
    /app/target/release/pulse-cast \
    /usr/local/bin/

EXPOSE 8080

CMD ["/usr/local/bin/pulse-cast"]