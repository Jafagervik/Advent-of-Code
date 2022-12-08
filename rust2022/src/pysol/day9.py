import numpy as np 

# Parse input to 2d array
grid = np.array([list(l.strip()) for l in open('../files/day9.txt')], int)

p1 = 0
p2 = 0

p11 = np.zeros_like(grid, int)
p22 = np.ones_like(grid, int)

print(np.ndindex(grid.shape))


print(0 , 0)
