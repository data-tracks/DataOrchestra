#!/bin/bash

session=EnergyProcessor

tmux new-session -d -s $session

tmux send-keys -t $session "cd energy_processor" C-m
tmux send-keys -t $session "cargo run -- --consumer localhost:9092 --producer localhost:9092 --consumer-topic EnergyIn --producer-topic EnergyOut -l debug"