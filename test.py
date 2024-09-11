import requests
import threading
import time
jobs = 150

i = 0
start = time.time()
def job():
    global i
    while True:
        r = requests.get('http://172.16.0.4:3001/health')
        if r.status_code != 200:
            print('Error:', r.status_code)
        i += 1
        #print(f'{i}: {r.status_code}')

jobs = [threading.Thread(target=job) for _ in range(jobs)]
for job in jobs:
    job.start()

while True:
    time.sleep(1)
    print(f"Requests per second: {i / (time.time() - start)}")

for job in jobs:
    job.join()