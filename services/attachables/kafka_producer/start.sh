#!/bin/bash

session=KafkaProducer

tmux new-session -d -s $session

tmux send-keys -t $session "cd /kafka_producer" C-m
tmux send-keys -t $session "cargo run" C-m
