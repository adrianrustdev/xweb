FROM rust:latest

WORKDIR /usr/src/xweb
COPY . .

RUN cargo install --path .
CMD ["xweb"]