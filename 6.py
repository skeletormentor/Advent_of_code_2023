from common import read_file
import re
import math

def get_distances(time, record):
    winning_times = 0
    for speed in range(1, time//2+1):
        distance = (time - speed) * speed
        if distance > record:
            winning_times += 1
    winning_times *= 2
    if time % 2 == 0:
        winning_times -= 1
    return winning_times

def get_input1(filename):
    lines = read_file(filename)
    nums = []
    for line in lines:
        times = [int(d) for d in re.findall('\d+', line)]
        nums.append(times)
    return list(zip(nums[0], nums[1]))

def get_input2(filename):
    lines = read_file(filename)
    time = int(''.join(re.findall('\d+', lines[0])))
    record = int(''.join(re.findall('\d+', lines[1])))
    return (time, record)

if __name__ == "__main__":
    nums = get_input1('input/input6.txt')
    nums2 = get_input2('input/input6.txt')
    
    print(math.prod([get_distances(t, r) for t, r in nums]))
    print(get_distances(*nums2))
    