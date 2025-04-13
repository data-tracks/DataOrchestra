echo "Setting up venv"
apt-get install pip -y
apt install python3.11-venv -y
python3 -m venv venv
. venv/bin/activate

echo "Installing dependencies"
pip install -r requirements.txt

echo "Starting generator.py"
python3 generator.py
