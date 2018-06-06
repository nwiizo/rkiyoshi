#numpy 使うのしんどい
import random


zndk = 0
zndk_count = 0 
while True:
    cb = random.randint(0,1)
    zndk_count += 1
    if cb:
        print('ZUN ',end='')
        zndk += 1
    else:
        print('DOKO ',end='')
        if zndk >= 4:
            break
        zndk = 0

print('KIYOSHI!!!!!!!!')
print('count:{}'.format(zndk_count))
