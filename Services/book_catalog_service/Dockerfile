FROM rust:1.62.0-slim-buster AS base
WORKDIR /app
EXPOSE 5001

FROM rust:1.62.0-slim-buster AS build
WORKDIR /src
COPY . .
RUN apt-get update && apt-get install -y protobuf-compiler
RUN cargo install --path .

FROM base AS final
WORKDIR /app
COPY --from=build /src/target/release/book-catalog-service .
CMD ["./book-catalog-service"]
