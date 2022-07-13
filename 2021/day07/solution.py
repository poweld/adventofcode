from typing import Iterable, Tuple, List

def getLines(filename) -> Iterable[str]:
    with open(filename) as f:
        line = f.readline()
        while line:
            yield line[:-1]
            line = f.readline()

class LanternFish:
    def __init__(self, timer: int = 8):
        self.timer = timer

    def ageUp(self) -> bool:
        # return whether to reproduce
        #print(f"ageUp: current timer={self.timer}")
        if self.timer == 0:
            self.timer = 6
            #print(f"ageUp: new timer={self.timer}")
            return True
        self.timer -= 1
        #print(f"ageUp: new timer={self.timer}")
        return False

    def __str__(self):
        return f"{self.timer}"

    def __repr__(self):
        return self.__str__()

# def parseLines(lines: Iterable[str]) -> Iterable[LineSegment]:
#     for line in lines:
#         parts = line.split(" -> ")
#         coord1 = tuple(map(int, parts[0].split(",")))
#         coord2 = tuple(map(int, parts[1].split(",")))
#         yield LineSegment(coord1, coord2)

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
    positions = list(map(int, line.split(",")))
    from collections import defaultdict
    positionCounts = defaultdict(lambda: 0)
    minPosition = positions[0]
    maxPosition = minPosition
    for position in positions:
        positionCounts[position] += 1
        minPosition = min(minPosition, position)
        maxPosition = max(maxPosition, position)
    costs = []
    for dest in range(minPosition, maxPosition + 1):
        cost = 0
        for position, count in positionCounts.items():
            cost += abs(dest - position) * count
        costs.append(cost)
    return min(costs)

def part2(filename):
    lines = getLines(filename)
    line = next(lines)
    positions = list(map(int, line.split(",")))
    from collections import defaultdict
    positionCounts = defaultdict(lambda: 0)
    minPosition = positions[0]
    maxPosition = minPosition
    for position in positions:
        positionCounts[position] += 1
        minPosition = min(minPosition, position)
        maxPosition = max(maxPosition, position)

    from functools import lru_cache
    @lru_cache(maxsize=None)
    def costfn(delta: int) -> int:
        return sum(range(1, delta + 1))

    costs = []
    for dest in range(minPosition, maxPosition + 1):
        cost = 0
        for position, count in positionCounts.items():
            delta = abs(dest - position)
            moveCost = costfn(delta)
            cost += moveCost * count
        costs.append(cost)
    return min(costs)

if __name__ == "__main__":
    main()
