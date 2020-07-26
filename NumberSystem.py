import math

def createBase64Translator(translator):  # 62 characters
    for i in range(0, 26):
        translator.append(chr(i+65))    # A-Z
    for i in range(0, 26):
        translator.append(chr(97+i))    # a -z
    for i in range(0, 10):
        translator.append(str(i))       # 1-10
    translator.append('+')
    translator.append('/')

def createBase64Decoder(decoder):
    for i in range(0, 26):
        decoder[chr(i+65)] = i    # A-Z
    for i in range(0, 26):
        decoder[chr(97+i)] = 26+i    # a -z
    for i in range(0, 10):
        decoder[str(i)] = 52+i       # 1-10
    decoder['+'] = 62
    decoder['/'] = 63


def numberBase(translator, number, base):
    power = getOutputLength(number, base)
    #print('power', power)
    result = ''
    while(power > -1):
        digit = math.pow(base, power)
        hiConstant = highestConstant(number, digit, base)
        #print(base,"^", power, "*", hiConstant,"\t=", digit*hiConstant)
        if(hiConstant <= number):
            number -= int(digit*hiConstant)
            result += translator[hiConstant]
        else:
            digit = 0
            result += '0'
        power -= 1
    return result



def highestConstant(number, digit, base):
    for i in range(1, base):
        if(digit*i > number):
            return i-1
        if(digit*i == number):
            return i
    return base -1

def getOutputLength(number, base):
    power = 0
    while(math.pow(base, power) <= number):
        power+=1
    if power is 0:
        return 1
    return power-1

def fillZeros(binary_input, divisible):
    while(len(binary_input)%divisible is not 0):
        binary_input = '0' + binary_input
    return binary_input

def padZeros(binary_input, divisible):
    while(len(binary_input)%divisible is not 0):
        binary_input = binary_input + '0'
    return binary_input


