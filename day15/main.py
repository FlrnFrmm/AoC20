from collections import defaultdict, deque

data = [int(vs) for vs in open('input.txt').read().split(',')]

numbers = defaultdict(int)
spoken = defaultdict(lambda: deque(maxlen=2))
last_num = 0
turn = 1
for num in data:
    numbers[num] += 1
    spoken[num].append(turn)
    turn += 1
    last_num = num

while turn <= 2020:
    if len(spoken[last_num]) != 2:
        numbers[0] += 1
        spoken[0].append(turn)
        last_num = 0
    else:
        last_num = max(spoken[last_num]) - min(spoken[last_num])
        numbers[last_num] += 1
        spoken[last_num].append(turn)
    turn += 1
print('Part 1:', last_num)

while turn <= 30_000_000:
    if len(spoken[last_num]) != 2:
        numbers[0] += 1
        spoken[0].append(turn)
        last_num = 0
    else:
        last_num = max(spoken[last_num]) - min(spoken[last_num])
        numbers[last_num] += 1
        spoken[last_num].append(turn)
    turn += 1
print('Part 2:', last_num)