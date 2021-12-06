import numpy as np


def expand(grid):
    new_shape = (grid.shape[0] + 2, grid.shape[1] + 2, grid.shape[2] + 2, grid.shape[3] + 2)
    new_grid = np.zeros(new_shape, bool)
    new_grid[1:-1, 1:-1, 1:-1, 1:-1] = grid
    return new_grid

def step(grid):

    grid = expand(grid)
    I, J, K, L = grid.shape
    print(I, J, K, L)

    next_grid = np.array(grid)

    for l in range(0, L):
        lm = max(0, l - 1)
        lp = min(l + 2, L)
        for k in range(0, K):
            km = max(0, k - 1)
            kp = min(k + 2, K)
            for j in range(0, J):
                jm = max(0, j - 1)
                jp = min(j + 2, J)
                for i in range(0, I):
                    on = int(grid[i, j, k, l])
                    im = max(0, i - 1)
                    ip = min(i + 2, I)
                    neighbors = grid[im:ip,jm:jp,km:kp,lm:lp]
                    ons = neighbors.sum() - on
                    if on == 1 and (ons != 2 and ons != 3):
                        on = 0
                    elif on == 0 and ons == 3:
                        on = 1
                    next_grid[i, j, k, l] = on

    return next_grid


def main(filename):
    with open(filename) as f:
        func = lambda c: c == "#"
        grid = [[func(c) for c in line.strip()] for line in f]
        grid = np.array(grid)
        new_grid = np.zeros((grid.shape[0], grid.shape[1], 1, 1), bool)
        new_grid[:, :, 0, 0] = grid[:, :]
        grid = new_grid

    for i in range(6):
        new_grid = step(grid)
        print(f"Step {i} {new_grid.sum()}")
        grid = new_grid


if __name__ == "__main__":
    main("input.txt")
