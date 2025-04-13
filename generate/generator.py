import time

import requests
from faker import Faker

fake = Faker()

while True:
    data = {"name": fake.name(), "email": fake.email()}
    print(data)
    time.sleep(0.5)
    requests.post("http://process:5000", json=data)
