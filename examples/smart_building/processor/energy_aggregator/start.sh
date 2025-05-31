#!/bin/bash

session=EnergyAggregator

tmux new-session -d -s $session

tmux send-keys -t $session "cd /energy_aggregator" C-m
tmux send-keys -t $session "cargo run -- --consumer broker:29092 --producer broker:29092 --consumer-topic EnergyFiltered --producer-topic EnergyAggregated -l debug" C-m
