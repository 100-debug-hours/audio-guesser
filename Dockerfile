# frontend
FROM node:13.6.0 AS frontend
WORKDIR /usr/src/frontend
COPY frontend .
RUN npm i && npm run-script build

# backend
FROM rust:1.40.0
WORKDIR /usr/src/backend
COPY --from=frontend /usr/src/frontend/dist dist
COPY backend .
RUN cargo build --release
CMD ["cargo", "run", "--release"]
