# Build frontend
FROM node:18-alpine AS frontend_builder
WORKDIR /app
COPY ./web/package.json ./web/yarn.lock .
RUN yarn install --frozen-lockfile
ADD ./web/ .
RUN yarn build

# Build backend
FROM rust:1.67-slim-buster as backend_builder
WORKDIR /usr/src/anthill
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
ADD ./migrations ./migrations
ADD ./src ./src
COPY ./Cargo.toml ./Cargo.lock ./diesel.toml .
RUN cargo install --path .

# Run
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
COPY --from=backend_builder /usr/local/cargo/bin/anthill /usr/local/bin/anthill
COPY --from=frontend_builder /app/dist /usr/local/share/anthill/web
EXPOSE 8080
CMD ["anthill", "--frontend-path", "/usr/local/share/anthill/web", "--address", "0.0.0.0"]
