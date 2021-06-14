import re

def generate_instruction(instruction_string):
    [head, tail] = instruction_string.split(" = ")
    if head == "mask":
        return {'cmd': 'setMask', 'value': tail}
    else:
        address = int(''.join([c for c in head if c.isdigit()]))
        return {'cmd': 'writeMemory', 'address': address,'value': int(tail)}

def generate_addresses(mask, address):
    address_string = bin(address)[2:].zfill(36)
    print(address_string)
    addresses = []
    if mask[0] == '0':
        addresses.append(address_string[0])
    if mask[0] == '1':
        addresses.append('1')
    if mask[0] == 'x':
        addresses.append('0')
        addresses.append('1')
    print(list(zip(mask[1:], address_string[1:])))
    for (m, v) in zip(mask[1:], address_string[1:]):
        if m == '0':
            addresses = [a + v for a in addresses]
        if m == '1':
            addresses = [a + '1' for a in addresses]
        if m == 'X':
            addresses = [a + e for a in addresses for e in ['0', '1']]
    return addresses

instructions = [generate_instruction(l) for l in open('input.txt').read().split('\n')]
print(instructions)

memory = {}
mask = '0' * 36

for instruction in instructions:
    if instruction['cmd'] == 'setMask':
        mask = instruction['value']
    if instruction['cmd'] == 'writeMemory':
        value = instruction['value']
        for a in generate_addresses(mask, instruction['address']):
            memory[int(a,2)] = value

print(memory)
sum = 0
for (k,v) in memory.items():
    sum += v
print(f"Sum: {sum}")