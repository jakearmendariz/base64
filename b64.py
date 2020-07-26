import sys
from encode import *
from decode import *

if __name__ == '__main__':
    result = ''
    if sys.argv[1] == 'encode':
        result = encode(sys.argv[2])
    else:
        result = decode(sys.argv[2])
    print(result)

