from collections import defaultdict
data = open('input.txt', 'r').read().split('\n')

bounds = []
d = {}

for line in data:
	if 'your' in line:
		break
	else:
		d[line.split(':')[0]] = None
		line_ = line.split('or ')
		lower_b = (line_[0].split(':')[1].split('-')[0],line_[0].split(':')[1].split('-')[1])
		upper_b = (line_[-1].split('-')[0], line_[-1].split('-')[1])
		bounds.append(lower_b)
		bounds.append(upper_b)
		d[line.split(':')[0]] = (lower_b, upper_b)


valid_tickets = []
total = 0
for ticket in data[23:]:
	ticket = ticket.split(',')
	add_valid = True
	for val in ticket:
		valid = False
		for b in bounds:
			if int(b[0]) <= int(val) <= int(b[1]):
				valid = True
				break
		if not valid:
			add_valid = False
			total += int(val)
	if add_valid:
		valid_tickets.append(ticket)

print(f'Part1: {total}')

fields = defaultdict(set)
for k in d:
	for i in range(len(valid_tickets[0])):
		candidate = True
		for j in range(len(valid_tickets)):
			val = int(valid_tickets[j][i])
			if (int(d[k][0][0]) <= val <= int(d[k][0][1])) or (int(d[k][1][0]) <= val <= int(d[k][1][1])):
				candidate = True
			else:
				candidate = False
				break
		if candidate:
			fields[k].add(i)

checked = []
search = True
while search:
	search = False
	for f in fields:
		if len(fields[f]) == 1 and f not in checked:
			x = fields[f].pop()
			fields[f].add(x)
			checked.append(f)
			for f_ in fields:
				if len(fields[f_]) > 1:
					if x in fields[f_]:
						fields[f_].remove(x)
						search = True

my_ticket = [int(x) for x in data[21].split(',')]
ans = 1
for k in fields:
	if 'depart' in k:
		x = int(fields[k].pop())
		ans *= my_ticket[x]

print(f'Part1: {ans}')