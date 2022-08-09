import string

from typing import Iterable, Tuple, List, Set, Dict
from pprint import pformat, pprint

def getLines(filename) -> Iterable[str]:
    with open(filename) as f:
        line = f.readline()
        while line:
            yield line[:-1]
            line = f.readline()

def parseLines(lines: Iterable[str]) -> Iterable[Tuple[int, int]]:
    for line in lines:
        if not line:
            return
        parts = line.split(",")
        parts.reverse()
        yield tuple(map(int, parts))

def main():
    part1TestResult = part1("testinput.txt")
    print(f"part1 test: {part1TestResult}")
    part1Result = part1("input.txt")
    print(f"part1: {part1Result}")

    # part2TestResult = part2("testinput.txt")
    # print(f"part2 test: {part2TestResult}")
    # part2Result = part2("input.txt")
    # print(f"part2: {part2Result}")

class Map():
    def __init__(self, coords: List[Tuple[int, int]]):
        self.rowCount = max(coord[0] for coord in coords) + 1
        self.colCount = max(coord[1] for coord in coords) + 1
        self._map = list()
        for _ in range(self.rowCount):
            self._map.append(["."] * self.colCount)
        for row, col in coords:
            # print(f"setting up map: row={row}, col={col}, rowCount={self.rowCount}, colCount={self.colCount}")
            self._map[row][col] = "#"

    def __str__(self) -> str:
        #return pformat(self._map)
        s = ""
        for row in self._map:
            for col in row:
                s += col
            s += "\n"
        return s

    def __repr__(self):
        return str(self)

    def fold(self, location: Tuple[str, int]) -> None:
        rowOrCol, foldAt = location
        newMap = list()
        if rowOrCol == "row":
            fold1 = self._map[:foldAt]
            fold2 = list(reversed(self._map[foldAt + 1:]))
            minLen = min(len(fold1), len(fold2))
            longerFold = fold1 if len(fold1) > len(fold2) else fold2
            while len(longerFold) > minLen:
                newMap.append(longerFold.pop(0))
            for i in range(len(fold1)):
                newRow = list()
                fold1Row = fold1[i]
                fold2Row = fold2[i]
                for j in range(len(fold1Row)):
                    if fold1Row[j] == "#" or fold2Row[j] == "#":
                        newRow.append("#")
                    else:
                        newRow.append(".")
                newMap.append(newRow)
        else:
            for _ in range(self.rowCount):
                newMap.append(list())
            fold1 = [row[:foldAt] for row in self._map]
            fold2 = [list(reversed(row[foldAt + 1:])) for row in self._map]
            minLen = min(len(fold1[0]), len(fold2[0]))
            longerFold = fold1 if len(fold1[0]) > len(fold2[0]) else fold2
            while len(longerFold[0]) > minLen:
                for rowIndex, row in enumerate(longerFold):
                    newMap[rowIndex].append(row.pop(0))
            for i in range(len(fold1)):
                fold1Row = fold1[i]
                fold2Row = fold2[i]
                for j in range(len(fold1Row)):
                    if fold1Row[j] == "#" or fold2Row[j] == "#":
                        newMap[i].append("#")
                    else:
                        newMap[i].append(".")
        self._map = newMap
        self.rowCount = len(self._map)
        self.colCount = len(self._map[0])

    def getDotCount(self) -> int:
        count = 0
        for row in self._map:
            for col in row:
                if col == "#":
                    count += 1
        return count

def part1(filename):
    lines = getLines(filename)
    coords = list(parseLines(lines))
    # pprint(coords)
    folds = list()
    for line in lines:
        parts = line.split(" ")
        parts = parts[-1].split("=")
        parts[1] = int(parts[1])
        if parts[0] == "x":
            parts[0] = "col"
        else:
            parts[0] = "row"
        folds.append(tuple(parts))
    m = Map(coords)
    print(m)
    # pprint(folds)
    #for fold in [folds[0]]:
    for fold in folds:
        print(f"applying fold={fold}")
        m.fold(fold)
        print(m)
    print(m)
    return m.getDotCount()

def part2(filename):
    pass

if __name__ == "__main__":
    main()
