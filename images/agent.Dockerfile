FROM ubuntu:latest
LABEL authors="mathieu"

WORKDIR agent

COPY ../agent .

ENTRYPOINT ["cargo", "run"]