myArr = bytearray([0xea] * (131072 * 2))


# for i in range(0, 131072 * 2):
#     myArr[i] = i % 256

myArr[0xfffc] = 0xea
myArr[0xfffd] = 0xea
myArr[0xfffe] = 0xea
myArr[0xffff] = 0xea
myArr[0xeaea] = 0xea
myArr[0xeaea + 1] = 0xea
myArr[0xeaea + 2] = 0xea
myArr[0xeaea + 3] = 0x4c
myArr[0xeaea + 4] = 0xea
myArr[0xeaea + 5] = 0xea

with open('out.bin', 'wb') as f:
    f.write(myArr)
