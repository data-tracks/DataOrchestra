#!/bin/bash

session=WordSplitter

tmux new-session -d -s $session

tmux send-keys -t $session "cd /word_splitter" C-m
tmux send-keys -t $session "cargo run -- --consumer broker:29092 --producer broker:29092 --consumer-topic Sentence --producer-topic Word -l debug" C-m
