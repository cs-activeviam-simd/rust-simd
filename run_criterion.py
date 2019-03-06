#! /usr/bin/python3

import subprocess
import os
import json
import sys

def parse_line(line, name):
    #  line = '     time:   [32.801 us 33.488 us 34.241 us]'
    line = line.split('time:')[1]
    words = line.split()
    # Parse the benchmark ID: name/size given as arg as if it is too long it is not provided on the same line
    benchmark = name[0]
    array_length = name[1][:-1]
    # parse the time unit to convert to ns
    unit = words[1]
    multiplier = 1
    if unit == 's':
        multiplier = 1000000000
    elif unit == 'ms':
        multiplier = 1000000
    elif unit == 'us':
        multiplier = 1000
    # Parse the results
    score = float(words[2])*multiplier
    confidencemin = float(words[0][1:])*multiplier
    confidencemax = float(words[4])*multiplier
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

def run(minsize, maxsize, filename, extraArgs):
    p = subprocess.Popen(['cargo', 'bench', '--', '--color=never'] + extraArgs, stdout=subprocess.PIPE, env=dict(os.environ,ARRAY_LENGTH_MIN=str(minsize) , ARRAY_LENGTH_MAX=str(maxsize)))
    results = []
    name = ''
    file = open(filename, 'w')
    file.write("[\n")
    first=True
    while True:
        line = p.stdout.readline().decode()
        if line == '' and p.poll() is not None:
            break
        if line:
            if 'Warming up' in line or 'Collecting' in line:
                name = line.split()[1].split('/')
                print(line, file=sys.stderr)
            elif 'time:' in line:
                if not first:
                    file.write(',\n')
                first = False
                file.write(json.dumps(parse_line(line, name), indent=4, sort_keys=True))
                file.flush()
    file.write('\n]')
    file.close()

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: run_criterion.py <minsize> <maxsize> <outfile> [extra cargo args]\nIf a size is less than 64, it is interpreted a a power of 2")
    else:
        minsize = int(sys.argv[1])
        if minsize < 64:
            minsize  = 1 << minsize
        maxsize = int(sys.argv[2])
        if maxsize < 64:
            maxsize  = 1 << maxsize
        run(minsize, maxsize, sys.argv[3], sys.argv[4:])
