#!/usr/bin/env python
import signal
import sqlite3
import sys
import time
import Adafruit_DHT

print "logging temperature"

conn = sqlite3.connect("/home/pi/weather/data.db")

print "got connection to database"

def signal_handler(sig, frame):
    conn.commit()
    conn.close()
    print('You pressed Ctrl+C!')
    sys.exit(0)

signal.signal(signal.SIGINT, signal_handler)

while True:
    c = conn.cursor()
    print "got cursor"
    humidity, temperature = Adafruit_DHT.read_retry(11, 4)
    print 'Temp: {0:0.1f} C  Humidity: {1:0.1f} %'.format(temperature, humidity)
    humidity2, temperature2 = Adafruit_DHT.read_retry(11, 3)
    print 'Temp: {0:0.1f} C  Humidity: {1:0.1f} %'.format(temperature2, humidity2)

    tempavg = (temperature + temperature2) / 2.000
    humavg = (humidity + humidity2) / 2.000

    c.execute("INSERT INTO temp(date, temp, humidity) VALUES(?, ?, ?)", [time.mktime(time.gmtime()), tempavg, humavg])
    conn.commit()
    time.sleep(60)

