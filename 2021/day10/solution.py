from typing import Iterable, Tuple, List

def getLines(filename) -> Iterable[str]:
    with open(filename) as f:
        line = f.readline()
        while line:
            yield line[:-1]
            line = f.readline()

# def parseLines(lines: Iterable[str]) -> Iterable[List[int]]:
#     for line in lines:
#         yield list(map(int, line))

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
    openerToCloser = {
        "(": ")",
        "[": "]",
        "{": "}",
        "<": ">",
    }
    syntaxScoreTable = {
        ")": 3,
        "]": 57,
        "}": 1197,
        ">": 25137,
    }
    openStack: List[str] = list()
    syntaxScore = 0
    for line in lines:
        for char in line:
            if char in openerToCloser.keys():
                openStack.append(char)
                continue
            openElem = openStack.pop()
            expected = openerToCloser[openElem]
            if char == expected:
                continue
            print(f"{line} - Expected {expected}, but found {char} instead.")
            syntaxScore += syntaxScoreTable[char]
            break
    return syntaxScore

def part2(filename):
    lines = getLines(filename)
    openerToCloser = {
        "(": ")",
        "[": "]",
        "{": "}",
        "<": ">",
    }
    autocompleteScoreTable = {
        ")": 1,
        "]": 2,
        "}": 3,
        ">": 4,
    }
    autocompleteScores: List[int] = list()
    for line in lines:
        autocompleteScore = 0
        openStack: List[str] = list()
        for char in line:
            if char in openerToCloser.keys():
                openStack.append(char)
                continue
            openElem = openStack.pop()
            expected = openerToCloser[openElem]
            if char == expected:
                continue
            break
        else:
            if openStack:
                while openStack:
                    openElem = openStack.pop()
                    autocompleteScore *= 5
                    autocompleteScore += autocompleteScoreTable[openerToCloser[openElem]]
                autocompleteScores.append(autocompleteScore)
    autocompleteScores.sort()
    print(autocompleteScores)
    return autocompleteScores[len(autocompleteScores) // 2]

if __name__ == "__main__":
    main()
