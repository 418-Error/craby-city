ARG RUST_VERSION=1.77
FROM rust:${RUST_VERSION}-buster AS dependency
WORKDIR /opt/craby_city

RUN mkdir -p src && echo "fn main() {}" >> src/main.rs

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo fetch

FROM dependency AS build

COPY src src
RUN --mount=type=cache,target=/opt/target/ \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml  \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock  \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/craby-city /bin/server

FROM debian:bullseye-slim AS final

ENV CITY_API_ADDR=$CITY_API_ADDR
ENV CITY_API_PORT=$CITY_API_PORT
ENV CITY_API_DB_URL=$CITY_API_DB_URL
ENV CITY_API_DB_USER=$CITY_API_DB_USER
ENV CITY_API_DB_PWD=$CITY_API_DB_PWD

# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "1000" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
ENTRYPOINT ["/bin/server"]