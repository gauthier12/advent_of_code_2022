from parse import *
import time
import sys
import copy
start = time.time()
filename = sys.argv[1]
print("===========================")
print("  Advent Of Code 2022")
print("  Day 05")
print("  Python version")
print("===========================")
print("In file {}".format(filename))
crate_piles = []
instructions = []
with open(filename, "r") as f:
    for line in f:
        if "[" in line:
            # This is a description of the crate piles
            pile_number = int((len(line) + 1)/4)
            for new_pile in range(len(crate_piles), pile_number):
                crate_piles.append([])
            for i_crate in range(0, pile_number):
                crate_piles[i_crate].append(line[4*i_crate+1])
        if "move" in line:
            # this is a move instruction
            parsed = parse("move {number:d} from {from:d} to {to:d}\n", line)
            instructions.append(
                {"number": parsed["number"], "from": parsed["from"]-1, "to": parsed["to"]-1})

    # Put the pile in the right order for pushing/popping
    for pile in crate_piles:
        pile.reverse()
        while pile[-1] == " ":
            pile.pop()

    # Copy the initial state and solve problem A
    crate_piles_a = copy.deepcopy(crate_piles)
    for inst in instructions:
        for i_crate in range(0, inst["number"]):
            # move the crate one by one
            cur_crate = crate_piles_a[inst["from"]].pop()
            crate_piles_a[inst["to"]].append(cur_crate)
    message_a = ""
    for pile in crate_piles_a:
        message_a += pile.pop()

    # Copy the initial state and solve problem B
    crate_piles_b = copy.deepcopy(crate_piles)
    for inst in instructions:
        # move the crate on the loaded crate pile
        loaded_crate_pile = []
        for i_crate in range(0, inst["number"]):
            # move the crate one by one
            cur_crate = crate_piles_b[inst["from"]].pop()
            loaded_crate_pile.append(cur_crate)
        # unload the loaded crate pile
        for i_crate in range(0, inst["number"]):
            # move the crate one by one
            cur_crate = loaded_crate_pile.pop()
            crate_piles_b[inst["to"]].append(cur_crate)
    message_b = ""
    for pile in crate_piles_b:
        message_b += pile.pop()
end = time.time()
print("Time elapsed in total is: {}ms".format((end - start)*1000))
print("Problem A : message : {}".format(message_a))
print("Problem B : message : {}".format(message_b))
