FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
RUN rustup target add x86_64-unknown-linux-gnu 
RUN apt-get update -y \
    && apt-get install -y openssl ca-certificates \
    && apt-get install -y lld clang pkg-config -y\
    && apt-get install -y ca-certificates libssl-dev musl-dev musl-tools
WORKDIR /app


FROM chef AS planner
COPY ./testcase-ftp /app/testcaseFTP
COPY ./primitypes /app/primitypes
WORKDIR /app/testcaseFTP
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/testcaseFTP/recipe.json /app/testcaseFTP/recipe.json
RUN apt-get update -y \
    && apt-get install -y openssl ca-certificates \
    && apt-get install -y lld clang pkg-config -y\
    && apt-get install -y ca-certificates libssl-dev musl-dev musl-tools

WORKDIR /app/testcaseFTP
RUN rustup target add x86_64-unknown-linux-gnu 
COPY ./primitypes /app/primitypes
RUN cargo chef cook --release --target x86_64-unknown-linux-gnu --recipe-path recipe.json
COPY ./testcase-ftp /app/testcaseFTP
RUN cargo build --release --target x86_64-unknown-linux-gnu --bin testcaseFTP

FROM bitnami/minideb:latest as end
COPY --from=builder /app/testcaseFTP/target/x86_64-unknown-linux-gnu/release/testcaseFTP /testcaseFTP
ENTRYPOINT ["/testcaseFTP"]
EXPOSE 2121

