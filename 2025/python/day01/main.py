INPUT = "../../../input/2024/day01/input.txt"
TEST_INPUT = "../../../input/2024/day01/test_input.txt"


def read_input():
    with open(INPUT) as file:
        lines = [line.rstrip() for line in file]
        return lines


def parse_input(input):
    split = [line.split() for line in input]
    return [(int(left_value), int(right_value)) for (left_value, right_value) in split]


def main():
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


if __name__ == "__main__":
    main()
