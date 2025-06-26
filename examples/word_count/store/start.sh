#!/usr/bin/bash

session=WordsStorePostgres

tmux new-session -d -s $session

tmux send-keys -t $session "cd /words_store_postgres" C-m
tmux send-keys -t $session "cargo run -- -l debug --consumer-topic EnergyAggregated --consumer 10.34.64.162:9092" C-m
