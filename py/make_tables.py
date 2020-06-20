#!/usr/bin/env python
import sqlite3

conn = sqlite3.connect("data.db")
conn.execute('''CREATE TABLE IF NOT EXISTS temp(date INTEGER, temp TEXT, humidity TEXT)''')
conn.commit()
conn.close()
