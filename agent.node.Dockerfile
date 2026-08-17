FROM rust:1.94-alpine AS compile
LABEL authors="mathieu"

WORKDIR /node

COPY ./agent ./agent
COPY ./engine ./engine

WORKDIR /node/agent

RUN apk add --no-cache openssl-dev

RUN cargo build --release

FROM docker:dind AS run

WORKDIR /agent

COPY --from=compile /node/agent/target/release/data_orchestra_agent .
RUN chmod +x ./data_orchestra_agent

CMD [ "./data_orchestra_agent" ]
