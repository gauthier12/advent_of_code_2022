from parse import *
import time
import sys
start = time.time()
filename = sys.argv[1]
print("===========================")
print("  Advent Of Code 2022")
print("  Day 04")
print("  Python version")
print("===========================")
print("In file {}".format(filename))
num_pair_a = 0
num_pair_b = 0
with open(filename, "r") as f:
    for line in f:
        parsed_line = parse("{:d}-{:d},{:d}-{:d}\n", line)
        min_1 = parsed_line[0]
        max_1 = parsed_line[1]
        min_2 = parsed_line[2]
        max_2 = parsed_line[3]
        # an interval is included if the min and the max is inside the other interval
        if (min_1>=min_2 and max_1<=max_2) or (min_2>=min_1 and max_2<=max_1):
            num_pair_a = num_pair_a + 1
        # if min AND max of an interval are under the min of the other, it doesn't overlap --> if not, it overlaps
        if not(((min_1 < min_2) and (max_1 < min_2)) or ((min_2 < min_1) and (max_2 < min_1))):
            num_pair_b += 1
end = time.time()
print("Time elapsed in total is: {}ms".format((end - start)*1000));
print("Problem A : number of included pair : {}".format(num_pair_a))
print("Problem B : number of overlapping pair : {}".format(num_pair_b))
