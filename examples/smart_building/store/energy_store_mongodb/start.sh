#!/usr/bin/bash

session=EnergyStoreMongodb

tmux new-session -d -s $session

tmux send-keys -t $session "cd /energy_store_mongodb" C-m
tmux send-keys -t $session "cargo run -- -l debug --consumer-topic EnergyFiltered --consumer 10.34.64.162:9092 --database testdb --collection testcollection --mongo-address mongodb:27017 --user mongo --password mongo" C-m
