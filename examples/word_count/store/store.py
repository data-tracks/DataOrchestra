import json

import psycopg
from flask import Flask, request
from kafka import KafkaConsumer


app = Flask(__name__)
conn = psycopg.connect("dbname=store user=postgres password=postgres")
cur = conn.cursor()
consumer = KafkaConsumer('words')
for msg in consumer:
    cur.execute("""
        INSERT INTO counts (word, count)
        VALUES (%s, 1) ON CONFLICT (word)
    DO
        UPDATE SET count = counts.count + 1
        """, (msg,))

@app.route("/", methods=["POST"])
def writer():
    word = json.dumps(request.get_json())
    word = word.replace("\\", "")
    word = word.replace(".", "")
    word = word.replace("\"", "")
    cur.execute("""
        INSERT INTO counts (word, count)
        VALUES (%s, 1)
        ON CONFLICT (word)
        DO UPDATE SET count = counts.count + 1
    """, (word,))
    conn.commit()
    return "ok"


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000)
