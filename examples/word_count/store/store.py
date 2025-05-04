"""
Dummy consumer which consumes kafka data and stores it into a postgres database
"""
#import psycopg
from kafka import KafkaConsumer

#conn = psycopg.connect("dbname=store user=postgres password=postgres")
#cur = conn.cursor()
consumer = KafkaConsumer(
    'words',
    bootstrap_servers='broker:29092'
)

for msg in consumer:
    print(msg)