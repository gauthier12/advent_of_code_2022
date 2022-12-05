#from parse import *
import re
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
prog = re.compile('[0-9]+')
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
            #parsed = parse("move {number:d} from {from:d} to {to:d}\n", line)
            # instructions.append(
            #    {"number": parsed["number"], "from": parsed["from"]-1, "to": parsed["to"]-1})
            parsed2 = prog.findall(line)
            instructions.append(
                (int(parsed2[0]), int(parsed2[1])-1, int(parsed2[2])-1))
    # Put the pile in the right order for pushing/popping
    for pile in crate_piles:
        pile.reverse()
        while pile[-1] == " ":
            pile.pop()

    # Copy the initial state and solve problem A
    crate_piles_a = copy.deepcopy(crate_piles)
    for ins_number, ins_from, ins_to in instructions:
        for i_crate in range(0, ins_number):
            # move the crate one by one
            crate_piles_a[ins_to].append(crate_piles_a[ins_from].pop())
    message_a = ''.join([str(pile[-1]) for pile in crate_piles_a])

    # Copy the initial state and solve problem B
    crate_piles_b = copy.deepcopy(crate_piles)
    for ins_number, ins_from, ins_to in instructions:
        # move the crate on the loaded crate pile
        long = len(crate_piles_b[ins_from])
        loaded_crate_pile = crate_piles_b[ins_from][long-ins_number:long]
        del crate_piles_b[ins_from][long-ins_number:long]
        # unload the loaded crate pile
        crate_piles_b[ins_to].extend(loaded_crate_pile)
    message_b = ''.join([str(pile[-1]) for pile in crate_piles_b])
end = time.time()
print("Time elapsed in total is: {}ms".format((end - start)*1000))
print("Problem A : message : {}".format(message_a))
print("Problem B : message : {}".format(message_b))
