# maybe my worst code yet
activated = []

def is_complete(grid):
    for row in grid:
        matchCount = 0
        for i in activated:
            if i in row:
                matchCount += 1

        if matchCount >= len(row):
            return True
            
    for colIndex in range(len(grid)):
        matchCount = 0
        for row in grid:
            if row[colIndex] in activated:
                matchCount += 1

        if matchCount >= len(grid):
            return True

    return False



def sum_unmarked(grid):
    sum = 0
    for row in grid:
        for cell in row:
            if cell not in activated:
                sum += cell

    return sum

with open('input.txt') as f:
    lines = f.read().split('\n')
    inputs = list(map(int, lines[0].split(',')))
    
    gridLines = lines[2:]
    grids = []

    for i in range(len(gridLines) // 6):
        grid = []
        for j in range(5):
            row = list(map(int, filter(lambda line: line != '', gridLines[i*6+j].split(' '))))
            grid.append(row)

        grids.append(grid)

    for i in inputs:
        activated.append(i)

        for grid in grids:
            if is_complete(grid):
                print(sum_unmarked(grid) * i)
                quit()