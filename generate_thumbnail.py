import random

import imageio

w = h = 52

thumb = [[[0, 255, 0] for _ in range(w)] for _ in range(h)]

for x in range(w//3 - 4, w//3 + 4):
    for y in range(h//3 - 4, h//3 + 4):
        thumb[x][y][1] = 30

for x in range(w//3 - 4, w//3 + 4):
    for y in range((2*h)//3 - 4, (2*h)//3 + 4):
        thumb[x][y][1] = 30

imageio.imwrite('resources/thumbnail.jpg', thumb)
