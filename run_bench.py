#! /usr/bin/python3

import subprocess
import os
import json
import sys

def parse_output(s, array_length):
    res = []
    for line in s.splitlines():
        d = parse_bench(line)
        if d != {}:
            d["params"] = {
                "ARRAY_LENGTH": str(array_length)
            }
            res.append(d)
    return res

def parse_bench(s):
    if 'bench' not in s:
        return {}
    words = s.split()
    benchmark = words[1][13:]
    # print(words)
    score = int(words[4].replace(',',''))
    scoreUnit = 'ns/op'
    scoreError = int(words[7][:-1].replace(',',''))
    scoreConfidence = [score - scoreError, score + scoreError]
    return {
        'benchmark': benchmark,
        "primaryMetric": {
            'score': score,
            'scoreError': scoreError,
            'scoreConfidence': scoreConfidence,
            'scoreUnit': scoreUnit
        }
    }

def run(minsize, maxsize, extraArgs):
    results = []
    size = minsize
    while size <= maxsize:
        print('Running for ARRAY_LENGTH={}'.format(size))
        p = subprocess.run(['cargo', 'bench'] + extraArgs, stdout=subprocess.PIPE, env=dict(os.environ, ARRAY_LENGTH=str(size)))
        # print(p.stdout.decode())
        out = p.stdout.decode()
        results.extend(parse_output(out, size))
        size *= 2

    print(json.dumps(results, indent=4, sort_keys=True))

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: run_bench.py <minsize> <maxsize> [extra cargo args]\nIf a size is less than 64, it is interpreted a a power of 2")
    else:
        minsize = int(sys.argv[1])
        if minsize < 64:
            minsize  = 1 << minsize
        maxsize = int(sys.argv[2])
        if maxsize < 64:
            maxsize  = 1 << maxsize
        print(sys.argv[3:])
        run(minsize, maxsize, sys.argv[3:])
