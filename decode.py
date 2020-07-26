import sys
from NumberSystem import *

def decode(string):
    string = string.replace('=','')
    binary = base64ToBinary(string)
    #print('string in binary:', binary)
    return binaryStrToAscii(binary)

def binaryStrToAscii(binary_str):
    i = 0
    ascii_str = ''
    while(i <= len(binary_str)-8):
        section = binary_str[i:i+8]
        ascii_str += binaryDigitToAscii(section)
        i+=8
    #print("At the end, i == ",i,"length:", len(binary_str))
    return ascii_str


def binaryDigitToAscii(section):
    value = 0
    power= 7
    for i in range(len(section)):
        if section[i] == '1':
            value += int(math.pow(2, power))
        power -= 1
   # print('value:', value, 'ascii:', chr(value))
    return chr(value)

def base64ToBinary(input):
    result = ''
    decoder = {}
    createBase64Decoder(decoder)
    for i in range(len(input)):
        value = decoder[input[i]]
        #print("value", value, 'in binary:', base64DigitToBinary(value))
        result += base64DigitToBinary(value)
    return result

def base64DigitToBinary(value):
    translator = []
    translator.append('0')
    translator.append('1')
    return fillZeros(numberBase(translator, value, 2), 6)


if __name__ == '__main__':
    print(decode(sys.argv[1]))
