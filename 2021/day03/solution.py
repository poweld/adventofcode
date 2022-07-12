from typing import Iterable, Tuple, List

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
    part1Result = part1("input.txt")
    print(f"part1: {part1Result}")

    part2TestResult = part2("testinput.txt")
    print(f"part2 test: {part2TestResult}")
    part2Result = part2("input.txt")
    print(f"part2: {part2Result}")

def part1(filename):
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

def part2(filename):
    lines = list(getLines(filename))

    def getOxygen(lines: List[str], index: int = 0) -> str:
        if len(lines) == 1:
            return lines[0]
        zeroes = list()
        ones = list()
        for line in lines:
            if line[index] == "0":
                zeroes.append(line)
            else:
                ones.append(line)
        if len(ones) >= len(zeroes):
            return getOxygen(ones, index + 1)
        return getOxygen(zeroes, index + 1)

    def getCO2(lines: List[str], index: int = 0) -> str:
        if len(lines) == 1:
            return lines[0]
        zeroes = list()
        ones = list()
        for line in lines:
            if line[index] == "0":
                zeroes.append(line)
            else:
                ones.append(line)
        if len(zeroes) <= len(ones):
            return getCO2(zeroes, index + 1)
        return getCO2(ones, index + 1)

    oxygen = getOxygen(lines)
    oxygenInt = int(oxygen, 2)
    co2 = getCO2(lines)
    co2Int = int(co2, 2)
    product = oxygenInt * co2Int
    return f"oxygen={oxygen}, oxygenInt={oxygenInt}, co2={co2}, co2Int={co2Int}, product={product}"

if __name__ == "__main__":
    main()
