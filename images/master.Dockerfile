FROM rust:1.88
LABEL authors="mathieu"

WORKDIR master

COPY ../master .
COPY ../engine .
COPY ../parser .
