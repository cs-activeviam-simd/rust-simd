#! /usr/bin/python3

import subprocess
import os
import json

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


results = []

for i in range(9,27):
    array_length = 1 << i
    # print('Running for ARRAY_LENGTH={}'.format(array_length))
    p = subprocess.run(['cargo', 'bench'], capture_output=True, env=dict(os.environ, ARRAY_LENGTH=str(array_length)))
    # print(p.stdout.decode())
    out = p.stdout.decode()
    results.extend(parse_output(out, array_length))

print(json.dumps(results))


'''
2048

running 8 tests
test tests::test_add_simd ... ignored
test tests::test_mul_simd ... ignored
test tests::bench_add_reg       ... bench:       1,820 ns/iter (+/- 11)
test tests::bench_add_simd256   ... bench:         233 ns/iter (+/- 1)
test tests::bench_add_simd512   ... bench:       1,825 ns/iter (+/- 13)
test tests::bench_add_simd_rust ... bench:         255 ns/iter (+/- 2)
test tests::bench_mul_reg       ... bench:       1,853 ns/iter (+/- 8)
test tests::bench_mul_simd      ... bench:         262 ns/iter (+/- 2)

test result: ok. 0 passed; 0 failed; 2 ignored; 6 measured; 0 filtered out'''
