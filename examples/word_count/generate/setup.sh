apt-get install pip -y
apt install python3.11-venv -y

session="Generator"
tmux new-session -d -s $session
tmux send-keys -t $session "cd /generate" C-m
tmux send-keys -t $session "python3 -m venv venv" C-m
tmux send-keys -t $session ". venv/bin/activate" C-m
tmux send-keys -t $session "pip install -r requirements.txt" C-m
tmux send-keys -t $session "python3 generator.py" C-m
