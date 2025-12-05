def read_input():
    with open("input.txt", "r") as file:
        return file.read().split(",")


def parse_input(input):
    ranges = []
    for line in input:
        (range_start, range_end) = line.split("-")
        (range_start, range_end) = (int(range_start), int(range_end))
        ranges.append(range(range_start, range_end + 1))
    return ranges


def part_1():
    ranges = parse_input(read_input())
    total = 0
    for range_ in ranges:
        for value in range_:
            value_s = str(value)
            midpoint = len(value_s) // 2
            (a, b) = (value_s[:midpoint], value_s[midpoint:])
            if a == b:
                total += value
    print(total)


def chunk(lst, size):
    for i in range(0, len(lst), size):
        yield lst[i : i + size]


def part_2():
    ranges = parse_input(read_input())
    total = 0
    for range_ in ranges:
        for value in range_:
            value_s = str(value)
            valid = True
            for size in range(len(value_s) // 2, 0, -1):
                if len(value_s) % size == 0:
                    chunks = list(chunk(list(value_s), size))
                    first = chunks[0]
                    for c in chunks[1:]:
                        if c != first:
                            break
                    else:
                        valid = False
                        break
            if not valid:
                total += value
    print(total)


if __name__ == "__main__":
    part_1()
    part_2()
