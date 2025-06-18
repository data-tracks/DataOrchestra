#!/bin/bash

session=KafkaConsumer

tmux new-session -d -s $session

tmux send-keys -t $session "cd /kafka_consumer" C-m
tmux send-keys -t $session "cargo run" C-m
