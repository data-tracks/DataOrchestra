FROM rust:1.88
LABEL authors="mathieu"

WORKDIR agent

COPY ../agent .
COPY ../engine .

RUN cargo install --path .

ENTRYPOINT ["cargo", "run"]
