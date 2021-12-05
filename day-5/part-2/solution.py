def decode_pair(pair):
    split = pair.split(',')
    return (int(split[0]), int(split[1]))

grid = []
pairs = []
maxGridX = 0
maxGridY = 0

with open('input.txt') as f:
    lines = f.read().split('\n')
    for line in lines:
        pairsRaw = list(map(lambda s: s.strip(), line.split('->')))
        x1, y1 = decode_pair(pairsRaw[0])
        x2, y2 = decode_pair(pairsRaw[1])
        pairs.append((x1, y1, x2, y2))

        if max(x1, x2) > maxGridX:
            maxGridX = max(x1, x2)
        if max(y1, y2) > maxGridY:
            maxGridY = max(y1, y2)

    row = [0] * (maxGridX + 1)
    for i in range(maxGridY + 1):
        grid.append(row[:])

    for (x1, y1, x2, y2) in pairs:
        minX = min(x1, x2)
        minY = min(y1, y2)
        maxX = max(x1, x2)
        maxY = max(y1, y2)

        if y1 == y2:
            for x in range(minX, maxX + 1):
                grid[y1][x] += 1
        elif x1 == x2:
            for y in range(minY, maxY + 1):
                grid[y][x1] += 1
        else:
            # determine gradient
            if (y2 - y1) / (x2 - x1) > 0:
                xOffset = 0
                for y in range(minY, maxY + 1):
                    grid[y][minX+xOffset] += 1
                    xOffset += 1
            else:
                xOffset = 0
                for y in range(maxY, minY - 1, -1):
                    grid[y][minX+xOffset] += 1
                    xOffset += 1
    
    print(len([1 for row in grid for col in row if col > 1]))
