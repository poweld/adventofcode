from typing import Iterable, Tuple, List

def getLines(filename) -> Iterable[str]:
    with open(filename) as f:
        line = f.readline()
        while line:
            yield line[:-1]
            line = f.readline()

class LineSegment:
    def __init__(self, coord1, coord2):
        self.coord1 = coord1
        self.coord2 = coord2

    def coords(self) -> Iterable[Tuple[int, int]]:
        isHorizontal = self.coord1[0] == self.coord2[0]
        isVertical = self.coord1[1] == self.coord2[1]
        if isHorizontal:
            row = self.coord1[0]
            startCol = min(self.coord1[1], self.coord2[1])
            endCol = max(self.coord1[1], self.coord2[1]) + 1
            for col in range(startCol, endCol):
                yield (row, col)
        elif isVertical:
            col = self.coord1[1]
            startRow = min(self.coord1[0], self.coord2[0])
            endRow = max(self.coord1[0], self.coord2[0]) + 1
            for row in range(startRow, endRow):
                yield (row, col)
        else:
            slope = (self.coord2[1] - self.coord1[1]) // (self.coord2[0] - self.coord1[0])
            coord = min((self.coord1, self.coord2), key=lambda coord: coord[0])
            coordEnd = max((self.coord1, self.coord2), key=lambda coord: coord[0])
            while coord[0] <= coordEnd[0]:
                yield coord
                coord = (coord[0] + 1, coord[1] + slope)


    def __str__(self):
        return f"{self.coord1} -> {self.coord2}"

    def __repr__(self):
        return self.__str__()

def parseLines(lines: Iterable[str]) -> Iterable[LineSegment]:
    for line in lines:
        parts = line.split(" -> ")
        coord1 = tuple(map(int, parts[0].split(",")))
        coord2 = tuple(map(int, parts[1].split(",")))
        yield LineSegment(coord1, coord2)

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
    segments = list(parseLines(lines))
    # only looking at horizontal or vertical lines
    segments = [
        segment for segment in segments
        if
            segment.coord1[0] == segment.coord2[0]
        or
            segment.coord1[1] == segment.coord2[1]
    ]
    from collections import defaultdict
    board: Dict[Tuple[int, int], int] = defaultdict(lambda: 0)
    pointsWithMin2Overlap = 0
    for segment in segments:
        for coord in segment.coords():
            board[coord] += 1
            if board[coord] == 2:
                pointsWithMin2Overlap += 1
    return f"pointsWithMin2Overlap={pointsWithMin2Overlap}"

def part2(filename):
    lines = getLines(filename)
    segments = list(parseLines(lines))
    from collections import defaultdict
    board: Dict[Tuple[int, int], int] = defaultdict(lambda: 0)
    pointsWithMin2Overlap = 0
    for segment in segments:
        for coord in segment.coords():
            board[coord] += 1
            if board[coord] == 2:
                pointsWithMin2Overlap += 1

    return f"pointsWithMin2Overlap={pointsWithMin2Overlap}"

if __name__ == "__main__":
    main()
