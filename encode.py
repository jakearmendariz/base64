import sys
from NumberSystem import *
def encode(string):
    binary = strToBinary(string)
    binary = padZeros(binary, 6)
    base64 = binaryStrToBase64(binary)
    return addPadding(base64)

def addPadding(base64):
    while(len(base64)%3 is not 0):
        base64 += '='
    return base64

def binaryStrToBase64(binary_str):
    i = 0
    base64 = ''
    while(i <= len(binary_str)-6):
        section = binary_str[i:i+6]
        base64 += binaryDigitToBase64(section)
        i+=6
    return base64

def binaryDigitToBase64(section):
    translator = []
    createBase64Translator(translator)
    value = 0
    power= 5
    for i in range(len(section)):
        if section[i] == '1':
            value += int(math.pow(2, power))
        power -= 1
    #print('value:', value, 'base64:', translator[value])
    return translator[value]


def strToBinary(input):
    result = ''
    for i in range(len(input)):
        ascii_value = ord(input[i])
        result += intToBinary(ascii_value)
    return result

def intToBinary(ascii_value):
    translator = []
    translator.append('0')
    translator.append('1')
    return fillZeros(numberBase(translator, ascii_value, 2), 8)

if __name__ == '__main__':
    print(encode(sys.argv[1]))
