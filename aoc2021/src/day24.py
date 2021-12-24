import numpy as np

with open('../data/day24.txt', 'rt') as f:
    data = np.array(f.read().strip().split('\n')).reshape(14, -1).T

for i, row in enumerate(data):
    if len(np.unique(data[i])) > 1:
        print(i, [int(s.rsplit(' ', 1)[1]) for s in data[i]])


# Only (4, 5, 15) rows are different

# w                    (1~9)
# x = z % 26
# div z (1|26)
# x = x + *-7*
# x != w               (>=0)
# # y = 25 * x + 1
# z = z * (25 * x + 1) (>=0)
# y = (w + *8*) * x    (>=0)
# add z y


# [1, 1, 1, 1, 1, 26, 26, 1, 26, 1, 26, 26, 26, 26]
# [14, 14, 14, 12, 15, -12, -12, 12, -7, 13, -8, -5, -10, -7]
# [14, 2, 1, 13, 5, 5, 5, 9, 3, 13, 2, 1, 11, 8]
