#! /usr/bin/python3

import subprocess
import os

for i in range(9,12):
    size = 1 << i
    print(size)
    p = subprocess.run(["cargo", "bench", "--", "--nocapture"], capture_output=True, env=dict(os.environ, VECTOR_SIZE=str(size)))
    print(p.stdout.decode())
    out = p.stdout.decode()
