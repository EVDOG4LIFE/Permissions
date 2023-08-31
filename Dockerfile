# Build Rust backend
FROM rust:latest as rust-build
WORKDIR /usr/src/app
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN apt-get update && apt-get install -y libsqlite3-dev
RUN cargo build --release

# Build Node.js frontend
FROM node:14 as node-build
WORKDIR /usr/app
COPY ./frontend/package.json ./
RUN npm install
COPY ./frontend/ ./
RUN npm run build

# Final image
FROM debian:latest
WORKDIR /app
COPY --from=rust-build /usr/src/app/target/release/permissions /app/permissions
COPY --from=node-build /usr/app/build /app/frontend/build
RUN apt-get update && apt-get install -y libsqlite3-0 libssl-dev
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD [ "./permissions" ]