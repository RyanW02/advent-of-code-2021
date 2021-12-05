def decode_pair(pair):
    split = pair.split(',')
    return (int(split[0]), int(split[1]))

grid = []
pairs = []
maxX = 0
maxY = 0

with open('input.txt') as f:
    lines = f.read().split('\n')
    for line in lines:
        pairsRaw = list(map(lambda s: s.strip(), line.split('->')))
        x1, y1 = decode_pair(pairsRaw[0])
        x2, y2 = decode_pair(pairsRaw[1])
        pairs.append((x1, y1, x2, y2))

        if max(x1, x2) > maxX:
            maxX = max(x1, x2)
        if max(y1, y2) > maxY:
            maxY = max(y1, y2)

    row = [0] * (maxX + 1)
    for i in range(maxY + 1):
        grid.append(row[:])

    for (x1, y1, x2, y2) in pairs:
        if y1 == y2:
            for i in range(min(x1, x2), max(x1, x2) + 1):
                grid[y1][i] += 1
        elif x1 == x2:
            for i in range(min(y1, y2), max(y1, y2) + 1):
                grid[i][x1] += 1            

    count = 0
    for row in grid:
        for col in row:
            if col > 1:
                count += 1

    print(count)
