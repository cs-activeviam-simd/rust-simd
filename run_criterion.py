#! /usr/bin/python3

import subprocess
import os
import json
import sys

def parse_line(line):
    #  line = 'addSIMD256/262144       time:   [32.801 us 33.488 us 34.241 us]'
    words = line.split()
    # Parse the benchmark ID: name/size
    name = words[0].split('/')
    benchmark = name[0]
    array_length = name[1]
    # parse the time unit to convert to ns
    unit = words[3]
    multiplier = 1
    if unit == 's':
        multiplier = 1000000000
    elif unit == 'ms':
        multiplier = 1000000
    elif unit == 'us':
        multiplier = 1000
    # Parse the results
    score = float(words[4])*multiplier
    confidencemin = float(words[2][1:])*multiplier
    confidencemax = float(words[6])*multiplier
    scoreError = (confidencemax - confidencemin) / 2
    scoreConfidence = [confidencemin, confidencemax]
    return {
        'benchmark': benchmark,
        "primaryMetric": {
            'score': score,
            'scoreError': scoreError,
            'scoreConfidence': scoreConfidence,
            'scoreUnit': 'ns/op'
        },
        'params': {
            'ARRAY_LENGTH': array_length
        }
    }

def parse_output(s):
    res = []
    for line in s.splitlines():
        if 'time:' in line:
            res.append(parse_line(line))
    return res


def run(minsize, maxsize, extraArgs):
    size = minsize
    p = subprocess.run(['cargo', 'bench', '--', '--color=never'] + extraArgs, capture_output=True, env=dict(os.environ,ARRAY_LENGTH_MIN=str(minsize) , ARRAY_LENGTH_MAX=str(maxsize)))
    # print(p.stder.decode())
    out = p.stdout.decode()
    results = parse_output(out)

    print(json.dumps(results, indent=4, sort_keys=True))

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: run_criterion.py <minsize> <maxsize> [extra cargo args]\nIf a size is less than 64, it is interpreted a a power of 2")
    else:
        minsize = int(sys.argv[1])
        if minsize < 64:
            minsize  = 1 << minsize
        maxsize = int(sys.argv[2])
        if maxsize < 64:
            maxsize  = 1 << maxsize
        run(minsize, maxsize, sys.argv[3:])
