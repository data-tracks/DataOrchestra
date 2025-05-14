#!/bin/bash

session=$1
command=$2

tmux new-session -d -s $session
tmux send-keys -t $session "$2" C-m