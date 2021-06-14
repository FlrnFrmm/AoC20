dishes = [(i.split(' '), set(a[:-1].split(' ')[1:])) for i, a in [l.split(' (') for l in open('test_input.txt').read().split('\n')]]

incs = {}
for ingredients, allergens in dishes:
    for i in ingredients:
        if i in incs:
            incs[i].append(allergens)
        else:
            incs[i] = [allergens]


count = 0
for k in incs:
    print(k, 'could contains', incs[k], ' -> ',set.intersection(*incs[k]))
    if set.intersection(*incs[k]) == set():
        print(k, 'is free of allergenes')
        for ingredients, allergens in dishes:
            if k in ingredients:
                print('\t', k , 'is in', ingredients)
                count += 1

print(count)