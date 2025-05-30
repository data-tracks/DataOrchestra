#!/usr/bin/bash

session=EnergyStore

tmux new-session -d -s $session

tmux send-keys -t $session "cd /energy_store" C-m
tmux send-keys -t $session "cargo run -- -l debug --consumer-topic EnergyOut --consumer 10.34.64.162:9092 --database testdb --collection testcollection --mongo-address localhost:27071 --user mongo --password mongo" C-m
