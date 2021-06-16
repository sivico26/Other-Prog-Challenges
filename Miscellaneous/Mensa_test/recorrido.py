import numpy as np
from operator import mul
from itertools import product

dicc = {"A":(-1,0), "B":(1,0), "D":(0,1), "I":(0,-1)}

def recorrido(mat, coords, path=None):
    if path is None:
        path = set()
    path.add(coords)
    cell = mat[coords]
    if cell == "F":
        return path
    else:
        steps, direc = cell
        mod = tuple(map(mul, dicc[direc], [int(steps)]*2))
        new_coords = tuple(c + m for c,m in zip(coords, mod))
        return recorrido(mat, new_coords, path)

def find_button(mat):
    n,m = mat.shape
    goal = set(product(range(n), range(m)))
    for i in range(n):
        for j in range(m):
            if recorrido(mat, (i,j)) >= goal:
                print(f"La tecla inicial es {mat[i,j]} ({i},{j})")
