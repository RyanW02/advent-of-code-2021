previous = 0
isFirst = True
increaseCount = 0

with open('input.txt') as f:
    lines = list(map(int, f.read().split('\n')))
    for line in lines:
        if not isFirst and line > previous:
            increaseCount += 1
        
        previous = line
        isFirst = False
    
print(increaseCount)