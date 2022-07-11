from typing import Iterable, Tuple

def getLines(filename="input.txt") -> Iterable[str]:
    with open(filename) as f:
        line = f.readline()
        while line:
            yield line[:-1]
            line = f.readline()

def parseLine(line: str) -> Tuple[str, int]:
    parts = line.split(" ")
    return (parts[0], int(parts[1]))

def main():
    print(f"part1: {part1()}")
    print(f"part2: {part2()}")

def part1():
    lines = getLines()
    parsed = map(parseLine, lines)
    horizontal = 0
    vertical = 0
    for direction, amount in parsed:
        if direction == "forward":
            horizontal += amount
        elif direction == "down":
            vertical += amount
        else:
            vertical -= amount
    return f"horizontal={horizontal}, vertical={vertical}, product={horizontal * vertical}"

def part2():
    lines = getLines()
    parsed = map(parseLine, lines)
    horizontal = 0
    vertical = 0
    aim = 0
    for direction, amount in parsed:
        if direction == "forward":
            horizontal += amount
            vertical += aim * amount
        elif direction == "down":
            aim += amount
        else:
            aim -= amount
    return f"horizontal={horizontal}, vertical={vertical}, product={horizontal * vertical}"

if __name__ == "__main__":
    main()
