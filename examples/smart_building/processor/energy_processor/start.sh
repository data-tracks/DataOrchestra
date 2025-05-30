#!/bin/bash

session=EnergyProcessor

tmux new-session -d -s $session

tmux send-keys -t $session "cd /energy_processor" C-m
tmux send-keys -t $session "cargo run -- --consumer broker:29092 --producer broker:29092 --consumer-topic EnergyIn --producer-topic EnergyOut -l debug" C-m
