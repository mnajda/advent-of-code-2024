import sys
import re

INSTRUCTIONS_SET = r"(do\(\))|(don't\(\))|mul\((\d{1,3}),(\d{1,3})\)"

def part1(input):
    result = 0

    for instruction in re.findall(INSTRUCTIONS_SET, input):
        match instruction:
            case ("do()", _, _, _):
                pass
            case (_, "don't()", _, _):
                pass
            case (_, _,lhs, rhs):
                result += int(lhs) * int(rhs)

    print(f"Part 1 result is {result}")

def part2(input):
    multiply = True
    result = 0

    for instruction in re.findall(INSTRUCTIONS_SET, input):
        match instruction:
            case ("do()", _, _, _):
                multiply = True
            case (_, "don't()", _, _):
                multiply = False
            case (_, _,lhs, rhs):
                result += int(lhs) * int(rhs) if multiply else 0

    print(f"Part 2 result is {result}")

if __name__ == "__main__":
    path = sys.argv[1]
    with open(path, "r") as file:
        input = file.read()

        part1(input)
        part2(input)
