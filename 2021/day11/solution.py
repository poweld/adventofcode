from typing import Iterable, Tuple, List

def getLines(filename) -> Iterable[str]:
    with open(filename) as f:
        line = f.readline()
        while line:
            yield line[:-1]
            line = f.readline()

def parseLines(lines: Iterable[str]) -> Iterable[List[int]]:
    for line in lines:
        yield list(map(int, line))

class Coord:
    def __init__(self, row: int, col: int):
        self.row = row
        self.col = col

    def __str__(self) -> str:
        return str((self.row, self.col))

    def __repr__(self) -> Tuple[int, int]:
        return str(self)

    def __hash__(self):
        return hash((self.row, self.col))

    def getNeighbors(self, rowMax: int, colMax: int) -> Iterable["Coord"]:
        rows = range(self.row - 1, self.row + 2)
        cols = range(self.col - 1, self.col + 2)
        for row in rows:
            if row < 0 or row >= rowMax:
                continue
            for col in cols:
                if col < 0 or col >= colMax or (row == self.row and col == self.col):
                    continue
                yield Coord(row, col)

import pprint
class Octopuses:
    def __init__(self, octopuses: List[List[int]]):
        self.octopuses = octopuses
        self.rows = len(self.octopuses)
        self.cols = len(self.octopuses[0])
        self.numFlashes = 0
        self.numSteps = 0

    def step(self) -> bool:
        # returns whether all have flashed
        toFlash: List[Coord] = list()
        flashed: Set[Coord] = set()
        for row in range(self.rows):
            for col in range(self.cols):
                coord = Coord(row, col)
                flash = self.raiseEnergy(coord)
                if flash:
                    toFlash.append(coord)
        while toFlash:
            flashCoord = toFlash.pop()
            neighbors = flashCoord.getNeighbors(self.rows, self.cols)
            for neighbor in neighbors:
                flash = self.raiseEnergy(neighbor)
                if flash:
                    toFlash.append(neighbor)
            flashed.add(flashCoord)
        for f in flashed:
            self.drainEnergy(f)
        numFlashed = len(flashed)
        self.numFlashes += numFlashed
        self.numSteps += 1
        return numFlashed == (self.rows * self.cols)

    def raiseEnergy(self, coord):
        # return whether a flash should occur
        self.octopuses[coord.row][coord.col] += 1
        return self.octopuses[coord.row][coord.col] == 10

    def drainEnergy(self, coord):
        self.octopuses[coord.row][coord.col] = 0

    def __str__(self):
        return pprint.pformat(self.octopuses)

    def __repr__(self):
        return str(self)

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
    nums = list(parseLines(lines))

    octopuses = Octopuses(nums)
    for _ in range(100):
        octopuses.step()
    return octopuses.numFlashes

def part2(filename):
    lines = getLines(filename)
    nums = list(parseLines(lines))

    octopuses = Octopuses(nums)
    while True:
        allFlashed = octopuses.step()
        if allFlashed:
            break
    return octopuses.numSteps

if __name__ == "__main__":
    main()
