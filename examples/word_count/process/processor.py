import json

import requests
from flask import Flask, request

app = Flask(__name__)


@app.route("/", methods=["POST"])
def writer():
    data = json.dumps(request.get_json())
    words = data.split()
    for word in words:
        try:
            word = word.replace("\\", "")
            word = word.replace(".", "")
            word = word.replace("\"", "")
            requests.post("http://store:5000/", json=word)
        except:
            continue
    return "ok"


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000)
