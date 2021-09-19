FROM ubuntu:18.04
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential -y

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY ./discord_bot/ /app
RUN cargo build --release


FROM ubuntu:18.04
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential -y
WORKDIR /app

COPY --from=0 /app/.env /app
COPY --from=0 /app/target/release/awesome-discord-bot /app

CMD ./awesome-discord-bot
