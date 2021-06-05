FROM ubuntu:18.04
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential -y

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY ./source/ /app
RUN cargo build --release


FROM ubuntu:18.04
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential -y
WORKDIR /app

COPY /source/run_bot_and_server.sh /app
COPY --from=0 /app/.env /app
COPY --from=0 /app/target/release/ /app

COPY /source/templates/ /app/templates
COPY /source/static/ /app/static

CMD ./run_bot_and_server.sh