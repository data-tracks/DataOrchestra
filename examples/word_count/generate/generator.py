"""
Dummy generator (producer) which creates random sentences and sends them to kafka server
"""
import time
from faker import Faker
from kafka import KafkaProducer

fake = Faker()
producer = KafkaProducer(
    bootstrap_servers='broker:29092'
)

while True:
    try:
        data = fake.text()
        words = data.split(" ")
        for word in words:
            future=producer.send("words", word.encode('utf-8'))
            result = future.get(timeout=60)
        time.sleep(0.5)
    except:
        print("Unable to send data to kafka")


