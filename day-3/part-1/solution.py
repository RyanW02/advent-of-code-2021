gamma = []

with open('input.txt') as f:
    lines = f.read().split('\n')
    for line in lines:
        # make sure we have enough space in gamma
        if len(line) > len(gamma):
            for i in range(len(line) - len(gamma)):
                gamma.append(0)
        
        pos = 0
        for c in line:
            if c == '1':
                gamma[pos] += 1
            pos += 1

    gammaString = ''
    epsilonString = ''
    for i in range(len(gamma)):
        if gamma[i] > len(lines) - gamma[i]:
            gammaString += '1'
            epsilonString += '0'
        else:
            gammaString += '0'
            epsilonString += '1'

    gammaDenary = int(gammaString, 2)
    epsilonDenary = int(epsilonString, 2)

    print(gammaDenary * epsilonDenary)
        