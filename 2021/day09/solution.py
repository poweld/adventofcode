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
    heightMap = list(parseLines(lines))
    numRows = len(heightMap)
    numCols = len(heightMap[0])

    def getNeighbors(row, col) -> Iterable[int]:
        neighborRows: List[int] = [row]
        neighborCols: List[int] = [col]
        if row > 0:
            neighborRows.append(row - 1)
        if row < numRows - 1:
            neighborRows.append(row + 1)
        if col > 0:
            neighborCols.append(col - 1)
        if col < numCols - 1:
            neighborCols.append(col + 1)
        for nRow in neighborRows:
            for nCol in neighborCols:
                if nRow == row and nCol == col:
                    continue
                yield heightMap[nRow][nCol]

    lowPoints: List[int] = list()
    for rowIndex, row in enumerate(heightMap):
        for colIndex, value in enumerate(row):
            neighbors = list(getNeighbors(rowIndex, colIndex))
            #print(f"coord={rowIndex},{colIndex}, neighbors={neighbors}")
            if all((value < neighbor for neighbor in neighbors)):
                lowPoints.append(value)
    lowPointRiskLevels = [lowPoint + 1 for lowPoint in lowPoints]
    return sum(lowPointRiskLevels)

def part2(filename):
    lines = getLines(filename)
    heightMap = list(parseLines(lines))
    numRows = len(heightMap)
    numCols = len(heightMap[0])

    def getNeighborCoords(coord: Tuple[int, int]) -> Iterable[Tuple[int, int]]:
        row, col = coord
        neighborRows: List[int] = [row]
        neighborCols: List[int] = [col]
        if row > 0:
            yield (row - 1, col)
        if row < numRows - 1:
            yield (row + 1, col)
        if col > 0:
            yield (row, col - 1)
        if col < numCols - 1:
            yield (row, col + 1)

    def getNeighbors(coord: Tuple[int, int]) -> Iterable[int]:
        for row, col in getNeighborCoords(coord):
            yield heightMap[row][col]

    lowPoints: List[Tuple[int, int]] = list()
    for rowIndex, row in enumerate(heightMap):
        for colIndex, value in enumerate(row):
            neighbors = list(getNeighbors((rowIndex, colIndex)))
            #print(f"coord={rowIndex},{colIndex}, neighbors={neighbors}")
            if all((value < neighbor for neighbor in neighbors)):
                lowPoints.append((rowIndex, colIndex))
    basins: List[Set[Tuple[int, int]]] = list()
    for lowPoint in lowPoints:
        # print(f"lowPoint={lowPoint}")
        if any((lowPoint in basin for basin in basins)):
            # this point is already contained in a basin
            continue
        basin: Set[Tuple[int, int]] = set()
        coordsToCheck = [lowPoint]
        while coordsToCheck:
            # print(f"coordsToCheck={coordsToCheck}")
            coord = coordsToCheck.pop()
            if coord in basin:
                continue
            # print(f"coord={coord}")
            row, col = coord
            value = heightMap[row][col]
            if value == 9:
                continue
            basin.add(coord)
            #coordsToCheck += [coord for coord in getNeighborCoords(coord) if heightMap[coord[0]][coord[1]] != 9]
            coordsToCheck += list(getNeighborCoords(coord))
        basins.append(basin)
        # print(f"basin={basin}")
    basinSizes = list(map(len, basins))
    basinSizes.sort(reverse=True)
    from functools import reduce

    return reduce(lambda x, y: x * y, basinSizes[:3], 1)

if __name__ == "__main__":
    main()
