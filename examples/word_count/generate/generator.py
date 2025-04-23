"""
Dummy generator (producer) which creates random sentences and sends them to kafka server
"""

import time
from faker import Faker
from kafka import KafkaProducer

fake = Faker()
producer = KafkaProducer(bootstrap_server='process:5000')

while True:
    data = fake.text()
    future = producer.send('words', b'{data}')
    result = future.get(timeout=60)
    time.sleep(0.5)


