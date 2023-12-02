from common import read_file


def calibrate(inputfile: str) -> int:
    lines = read_file(inputfile)
    total_sum = 0
    for line in lines:
        nums = [char for char in line if char.isnumeric()]
        num = nums[0] + nums[-1]
        total_sum += int(num)
    return total_sum


def calibrate2(inputfile: str) -> int:
    convert = {'one': '1', 'two': '2', 'three': '3', 'four': '4', 'five': '5',
               'six': '6', 'seven': '7', 'eight': '8', 'nine': '9'}
    lines = read_file(inputfile)
    total_sum = 0
    for line in lines:
        nums = []
        line_length = len(line)
        for i in range(line_length):
            for numtext in convert.keys():
                if line[i:line_length + 1].startswith(numtext):
                    nums.append(convert[numtext])
            if line[i].isnumeric():
                nums.append(line[i])
        num = nums[0] + nums[-1]
        total_sum += int(num)
    return total_sum


if __name__ == '__main__':
    print(calibrate('input/input1.txt'))
    print(calibrate2('input/input1.txt'))
