windows = []
increaseCount = 0

with open('input.txt') as f:
    lines = list(map(int, f.read().split('\n')))

    for i in range(len(lines) - 2):
        windows.append(lines[i] + lines[i+1] + lines[i+2])

increaseCount = 0
for i in range(1, len(lines) - 2):
    if windows[i] > windows[i-1]:
        increaseCount += 1

print(increaseCount)