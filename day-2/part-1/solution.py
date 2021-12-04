horizontal = 0
depth = 0

with open('input.txt') as f:
    lines = f.read().split('\n')
    for line in lines:
        split = line.split(' ')
        instruction = split[0]
        factor = int(split[1])

        if instruction == 'forward':
            horizontal += factor
        elif instruction == 'up':
            depth -= factor
        elif instruction == 'down':
            depth += factor

print(horizontal * depth)