import numpy as np

def rot90(matrix, clockwise = True, inplace = False):
    n = matrix.shape[0] - 1
    mat = matrix.copy()
    
    for i in range(n + 1):
        if clockwise:
            mat[:, n-i] = matrix[i,:]
        else:
            mat[::-1, i] = matrix[i, :]
    
    if inplace:
        matrix[:] = mat
    else:
        return mat

def ext_by_cyc(n):
    first = [[(0,None), (1, None),(1, None), (1, -1)]]
    return first + [[(i, -i), (i+1, -i), (i+1, -i), (i+1, -i-1)] for i in range(1, n)]

def rot45(matrix, clockwise = True, inplace = False):
    mat = matrix.copy()
    n_rings = matrix.shape[0] // 2
    ring_slices = ext_by_cyc(n_rings)
    rings = [[] for _ in range(n_rings)]
    
    ## extract
    for c in range(4):
        for r in range(n_rings):
            s = slice(*ring_slices[r][c])
            rings[r].extend(matrix[r,s].tolist())
            
        rot90(matrix, clockwise = False, inplace = True)
            
    ## Prepare
    shifts = ((matrix.shape[0] -i*2) // 2 for i in range(n_rings))
    if clockwise:
        new_rings = [np.roll(array, shift) for array, shift in zip(map(np.array, rings), shifts)]
    else:
        new_rings = [np.roll(array, -1*shift) for array, shift in zip(map(np.array, rings), shifts)]
    
    ## Assign
    n_items = [0]*4
    for c in range(4):
        for r in range(n_rings):
            s = slice(*ring_slices[r][c])
            size = matrix[r,s].size
            mat[r,s] = new_rings[r][n_items[r]:(n_items[r] + size)]
            n_items[r] += size
        rot90(mat, clockwise = False, inplace = True)
    
    if inplace:
        matrix[:] = mat
    else:
        return mat

def rotate(matrix, angle):
    matrix = np.array(matrix)
    sign = angle > 0
    angle = abs(angle) % 360
    if res := angle % 45 != 0:
        raise ValueError(f"{res} is not a valid rotation.")
    
    n_turns = angle // 45 
    for _ in range(n_turns // 2):
        rot90(matrix, clockwise = sign, inplace = True)
    
    if n_turns % 2 == 1:
        rot45(matrix, clockwise = sign, inplace = True)
    
    return matrix
