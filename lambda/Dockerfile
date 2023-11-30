FROM ghcr.io/cargo-lambda/cargo-lambda

WORKDIR /lambda

COPY src src
COPY Cargo.* .

RUN cargo lambda build -r --arm64 -F lambda
