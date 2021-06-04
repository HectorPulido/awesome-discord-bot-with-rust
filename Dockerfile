FROM ubuntu:18.04
RUN apt-get update && apt-get install curl -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
COPY ./source/ /app
RUN cargo build --release


FROM ubuntu:18.04
COPY --from=0 /app/.env .
COPY --from=0 /app/target/release/bot .
COPY --from=0 /app/target/release/server .
COPY run_bot_and_server.sh .

CMD ./run_bot_and_server.sh