FROM rust:1.98.1-alpine as builder
WORKDIR /usr/src/insanely-floaty
COPY . .
RUN cargo +nightly install --locked --path .

FROM alpine/socat:1.8.1.3
COPY --from=builder /usr/local/cargo/bin/insanely-floaty /usr/local/bin/insanely-floaty
COPY flag.txt /
ENTRYPOINT ["socat", "TCP-LISTEN:1337,fork", "EXEC:/usr/local/bin/insanely-floaty"]
