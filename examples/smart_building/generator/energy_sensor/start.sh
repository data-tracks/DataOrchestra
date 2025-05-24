#!/bin/bash

session=EnergySensor

tmux new-session -d -s $session

tmux send-keys -t $session "cd energy_sensor" C-m
tmux send-keys -t $session "cargo run -- -l debug -t EnergyIn --address 10.34.64.162:9092 --interval 1"