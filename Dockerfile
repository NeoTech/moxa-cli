FROM rust:1.68-slim-bullseye as MOXA
WORKDIR /app
COPY moxa-cli-src moxa-cli-src
RUN cd moxa-cli-src && cargo clean
RUN cd moxa-cli-src && RUSTFLAGS='-C link-args=-s' cargo build --bin moxa-cli --release

FROM node:19-bullseye-slim
WORKDIR /app
COPY package.json ./
RUN npm install
COPY index.js index.js
COPY --from=MOXA /app/moxa-cli-src/target/release/moxa-cli /app/moxa-cli
RUN mkdir -p public uploads
COPY public/index.html public/index.html
RUN chmod 777 public uploads
RUN chmod 777 moxa-cli
EXPOSE 8080
CMD ["npm", "start"]