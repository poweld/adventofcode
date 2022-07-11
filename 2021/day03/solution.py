from typing import Iterable, Tuple

def getLines(filename) -> Iterable[str]:
    with open(filename) as f:
        line = f.readline()
        while line:
            yield line[:-1]
            line = f.readline()

# def parseLine(line: str) -> Tuple[str, int]:
#     parts = line.split(" ")
#     return (parts[0], int(parts[1]))

def main():
    part1TestResult = part1("testinput.txt")
    print(f"part1 test: {part1TestResult}")
    part1Result = part1()
    print(f"part1: {part1Result}")

    part2TestResult = part2("testinput.txt")
    print(f"part2 test: {part2TestResult}")
    part2Result = part2()
    print(f"part2: {part2Result}")

def part1(filename="input.txt"):
    lines = getLines(filename)
    line = next(lines)
    bitSum = list()
    for char in line:
        if char == "0":
            bitSum.append(-1)
        else:
            bitSum.append(1)
    for line in lines:
        for index, char in enumerate(line):
            if char == "0":
                bitSum[index] -= 1
            else:
                bitSum[index] += 1
    gamma = "0b"
    epsilon = "0b"
    for bs in bitSum:
        if bs > 0:
            gamma = gamma + "1"
            epsilon = epsilon + "0"
        else:
            gamma = gamma + "0"
            epsilon = epsilon + "1"
    gammaInt = int(gamma, 2)
    epsilonInt = int(epsilon, 2)
    product = gammaInt * epsilonInt
    return f"gamma={gamma}, gammaInt={gammaInt}, epsilon={epsilon}, epsilonInt={epsilonInt}, product={product}"

def part2(test=False):
    pass

if __name__ == "__main__":
    main()
