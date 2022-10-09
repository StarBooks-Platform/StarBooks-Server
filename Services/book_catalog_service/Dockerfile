FROM rust:1.62.0-slim-buster AS build

ENV CARGO_TERM_COLOR always

# create empty project for caching dependencies
RUN USER=root cargo new --bin book-catalog-service
WORKDIR /book-catalog-service
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./

# cache dependencies
RUN cargo install --path . --locked
COPY . .
RUN touch src/main.rs
RUN apt-get update && apt-get install -y protobuf-compiler
RUN cargo install --path . --locked

FROM debian:buster-slim
EXPOSE 50051
COPY --from=build /usr/local/cargo/bin/book-catalog-service /usr/local/bin/book-catalog-service
CMD ["book-catalog-service"]