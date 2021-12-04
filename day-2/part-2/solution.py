horizontal = 0
depth = 0
aim = 0

with open('input.txt') as f:
    lines = f.read().split('\n')
    for line in lines:
        split = line.split(' ')
        instruction = split[0]
        factor = int(split[1])

        if instruction == 'forward':
            horizontal += factor
            depth += aim * factor
        elif instruction == 'up':
            aim -= factor
        elif instruction == 'down':
            aim += factor

print(horizontal * depth)