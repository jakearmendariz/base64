from encode import *
from decode import *
import sys

def encodeNTimes(input, n):
    for i in range(n):
        input = encode(input)
        # print(input)
    return input

def decodeNTimes(input, n):
    for i in range(n):
        input = decode(input)
        # print(input)
    return input


if __name__ == '__main__':
    input = 'Jake Armendariz'
    if len(sys.argv) > 1:
        input = sys.argv[1]
    result = encodeNTimes(input, 35)
    result = decodeNTimes(result, 35)

