import re
import sys
from typing import List, Tuple


def parse_input(input_file: str) -> List[Tuple[Tuple[int, int], Tuple[int, int]]]:
    rx = re.compile('[,-]')

    with open(input_file) as f:
        range_pairs = []
        for line in f:
            nums = list(map(int, rx.split(line.strip())))
            range_pairs.append(((nums[0], nums[1]), (nums[2], nums[3])))

        return range_pairs


def main():
    range_pairs = parse_input(sys.argv[1])

    part1_solution = 0
    part2_solution = 0
    for (start1, end1), (start2, end2) in range_pairs:
        if (start1 >= start2 and end1 <= end2) or (start2 >= start1 and end2 <= end1):
            part1_solution += 1

        if (start2 <= start1 <= end2) or (start1 <= start2 <= end1):
            part2_solution += 1

    print(f"Part 1 solution = {part1_solution}")
    print(f"Part 2 solution = {part2_solution}")


if __name__ == '__main__':
    main()
