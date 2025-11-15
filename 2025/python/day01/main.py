from collections import Counter


def read_input():
    with open("input.txt") as file:
        lines = [line.rstrip() for line in file]
        return lines


def parse_input(input):
    split = [line.split() for line in input]
    return [(int(left_value), int(right_value)) for (left_value, right_value) in split]


def part_1():
    parsed_input = parse_input(read_input())
    acc = ([], [])
    [
        acc := (acc[0] + [left_value], acc[1] + [right_value])
        for (left_value, right_value) in parsed_input
    ]
    (left, right) = acc
    left.sort()
    right.sort()
    distances = [abs(pair[0] - pair[1]) for pair in zip(left, right)]
    print(sum(distances))


def part_2():
    parsed_input = parse_input(read_input())
    acc = ([], [])
    [
        acc := (acc[0] + [left_value], acc[1] + [right_value])
        for (left_value, right_value) in parsed_input
    ]
    (left, right) = acc
    right_count = Counter()
    for right_value in right:
        right_count[right_value] += 1
    left.sort()
    right.sort()
    distances = [left_value * right_count[left_value] for left_value in left]
    print(sum(distances))


if __name__ == "__main__":
    part_1()
    part_2()
