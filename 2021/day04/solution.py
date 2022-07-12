from typing import Iterable, Tuple, List

def getLines(filename) -> Iterable[str]:
    with open(filename) as f:
        line = f.readline()
        while line:
            yield line[:-1]
            line = f.readline()

class Board:
    BOARD_SIZE = 5

    def __init__(self, nums: List[List[int]]):
        self.nums = nums
        self.lookup = dict()
        for rowIndex, row in enumerate(nums):
            for colIndex, num in enumerate(row):
                self.lookup[num] = (rowIndex, colIndex)

    def hit(self, value) -> bool:
        if value in self.lookup:
            hitRow, hitCol = self.lookup[value]
            self.nums[hitRow][hitCol] = -1
            for row in range(Board.BOARD_SIZE):
                if self.nums[row][hitCol] != -1:
                    break
            else:
                return True
            for col in range(Board.BOARD_SIZE):
                if self.nums[hitRow][col] != -1:
                    break
            else:
                return True
        return False

    def score(self) -> int:
        _score = 0
        for row in self.nums:
            for num in row:
                if num != -1:
                    _score += num
        return _score

    def __repr__(self):
        return self.nums

    def __str__(self):
        return str(self.nums)

def parseLines(lines: Iterable[str]) -> Iterable[Board]:
    from itertools import islice
    for _ in lines:
        nums = list()
        for line in islice(lines, 5):
            numsRow = list(map(int, line.split()))
            nums.append(numsRow)
        yield Board(nums)

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
    draws = list(map(int, next(lines).split(",")))
    boards = list(parseLines(lines))
    for draw in draws:
        for board in boards:
            if board.hit(draw):
                return board.score() * draw

def part2(filename):
    lines = getLines(filename)
    draws = map(int, next(lines).split(","))
    boards = list(parseLines(lines))
    for draw in draws:
        winningBoardIndices = list()
        for boardIndex, board in enumerate(boards):
            if board.hit(draw):
                winningBoardIndices.append(boardIndex)
        # print(f"winningBoardIndices={winningBoardIndices}")
        for boardIndex in reversed(winningBoardIndices):
            del boards[boardIndex]
        if len(boards) == 1:
            break
    board = boards[0]
    for draw in draws:
        if board.hit(draw):
            return board.score() * draw

if __name__ == "__main__":
    main()
