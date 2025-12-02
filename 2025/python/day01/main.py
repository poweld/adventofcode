def read_input():
    with open("input.txt") as file:
        lines = [line.rstrip() for line in file]
        return lines


# return: [(direction, amount)]
def parse_input(input):
    return [(line[0], int(line[1:])) for line in input]


def part_1():
    parsed_input = parse_input(read_input())
    dial = 50
    password = 0
    for direction, amount in parsed_input:
        vector = amount if direction == "R" else -1 * amount
        dial = (dial + vector) % 100
        if dial == 0:
            password += 1
    print(password)


def part_2():
    parsed_input = parse_input(read_input())
    dial = 50
    password = 0
    for direction, amount in parsed_input:
        remaining = amount if direction == "R" else -1 * amount
        while remaining != 0:
            if remaining > 0:
                dial = (dial - 1) % 100
                remaining -= 1
            else:
                dial = (dial + 1) % 100
                remaining += 1
            if dial == 0:
                password += 1
    print(password)


if __name__ == "__main__":
    part_1()
    part_2()
