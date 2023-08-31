# Permissions

## Status: Work in Progress

## Overview

This project is an attempt to build a full-stack application using Rust for the backend, Node.js for the frontend, and Docker for containerization for a self service Permissions tool in an enterprise environment. 

### Components

- **Backend**: Rust
    - Framework: Rocket
    - Database: SQLite
    - Other Libraries: `jsonwebtoken`, `ldap3`
- **Frontend**: Node.js
    - Framework: React
- **Containerization**: Docker

## Current State

### Completed

- Basic setup of Rust backend with Rocket framework.
- Dockerization of Rust backend.
- Setup JWT and LDAP (though LDAP is not fully functional).
- SQL connection established.

### Issues

- LDAP setup is failing due to the absence of an Active Directory.
- There were initial Docker build issues, specifically with library dependencies like SQLite and SSL.
- The application is not sending any data when accessed via a browser.

## Next Steps

1. **Debug LDAP Setup**: Either ensure that LDAP is correctly set up or handle its absence gracefully in the code.
2. **Docker Build Issues**: Update the Dockerfile to correctly handle dependencies.
3. **Application Response**: Debug why the application isn't sending any data on being accessed via a browser.
4. **Frontend**: Finalize the frontend part of the application, ensure it can communicate with the backend.
5. **Testing**: Once all components are in place, perform end-to-end testing to ensure everything is working as expected.

## Dockerfile

For reference, the Dockerfile used for the project is attached below.

```Dockerfile
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
FROM debian:buster
WORKDIR /app
COPY --from=rust-build /usr/src/app/target/release/permissions /app/permissions
COPY --from=node-build /usr/app/build /app/frontend/build
RUN apt-get update && apt-get install -y libsqlite3-0 libssl-dev
EXPOSE 8000
CMD [ "./permissions" ]
```
