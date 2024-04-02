from common import read_file
import numpy as np
import re
import itertools
import matplotlib.pyplot as plt
from matplotlib import collections as mc

lines = read_file('input/input24.test')
line_segments = []
test_area = (7, 27)
for line in lines:
    px, py, _, dx, dy, _ = [int(num) for num in re.findall(r'(-?\d+)', line)]
    p2x = px
    p2y = py
    while test_area[0] <= p2x <= test_area[1] and test_area[0] <= p2y <= test_area[1]:
        p2x += dx
        p2y += dy
    line_segments.append([(px, py), (p2x, p2y)])

lc = mc.LineCollection(line_segments)
fig, ax = plt.subplots()
ax.add_collection(lc)
ax.autoscale()
plt.show()