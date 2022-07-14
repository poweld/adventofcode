from typing import Iterable, Tuple, List

def getLines(filename) -> Iterable[str]:
    with open(filename) as f:
        line = f.readline()
        while line:
            yield line[:-1]
            line = f.readline()

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
    numToSegments: Dict[int, Set[str]] = {
        0: set("abcefg"),
        1: set("cf"),
        2: set("acdeg"),
        3: set("acdfg"),
        4: set("bcdf"),
        5: set("abdfg"),
        6: set("abdefg"),
        7: set("acf"),
        8: set("abcdefg"),
        9: set("abcdfg"),
    }
    uniqueSegmentLengthToValue = {
        2: 1,
        4: 4,
        3: 7,
        7: 8,
    }
    lines = getLines(filename)
    found = 0
    for line in lines:
        cipher1, cipher2 = line.split("|")
        cipher1Segments = list(map(set, cipher1.split()))
        cipher2Segments = list(map(set, cipher2.split()))
        for segmentSet in cipher2Segments:
            segmentLen = len(segmentSet)
            uniqueSegmentValue = uniqueSegmentLengthToValue.get(segmentLen)
            if uniqueSegmentValue is not None:
                found += 1
    return found


def part2(filename):
    numToSegments: Dict[int, Set[str]] = {
        0: set("abcefg"),
        1: set("cf"),
        2: set("acdeg"),
        3: set("acdfg"),
        4: set("bcdf"),
        5: set("abdfg"),
        6: set("abdefg"),
        7: set("acf"),
        8: set("abcdefg"),
        9: set("abcdfg"),
    }
    uniqueSegmentLengthToValue = {
        2: 1,
        4: 4,
        3: 7,
        7: 8,
    }
    lines = getLines(filename)
    from collections import defaultdict
    resultSum = 0
    for line in lines:
        segmentSetCipher: Dict[int, Set[str]] = dict()
        cipher: Dict[str, str] = dict()
        cipher1, cipher2 = line.split("|")
        cipher1Segments = list(map(set, cipher1.split()))
        cipher2Segments = list(map(set, cipher2.split()))
        segmentSetsByLength: Dict[int, List[Set[str]]] = defaultdict(list)
        for segmentSet in cipher1Segments:
            segmentLen = len(segmentSet)
            uniqueSegmentValue = uniqueSegmentLengthToValue.get(segmentLen)
            if uniqueSegmentValue is not None:
                segmentSetCipher[uniqueSegmentValue] = segmentSet
            segmentSetsByLength[segmentLen].append(segmentSet)
        cipher["a"] = list(segmentSetCipher[7] - segmentSetCipher[1])[0]
        for segmentSet in segmentSetsByLength[4]:
            if cipher["a"] not in segmentSet:
                segmentSetCipher[4] = segmentSet
        for segmentSet in segmentSetsByLength[6]:
            segmentLen = len(segmentSet)
            # if 4 fully overlaps, then we found 9
            if len(segmentSet - segmentSetCipher[4]) == segmentLen - 4:
                segmentSetCipher[9] = segmentSet
            # else if 7 fully overlaps, then we found 0
            elif len(segmentSet - segmentSetCipher[7]) == segmentLen - 3:
                segmentSetCipher[0] = segmentSet
            # else we found 6
            else:
                segmentSetCipher[6] = segmentSet
        for segmentSet in segmentSetsByLength[5]:
            segmentLen = len(segmentSet)
            # if 7 fully overlaps, we found 3
            if len(segmentSet - segmentSetCipher[7]) == segmentLen - 3:
                segmentSetCipher[3] = segmentSet
            # if subtracting 6 yields the empty set, we found 5
            elif len(segmentSet - segmentSetCipher[6]) == 0:
                segmentSetCipher[5] = segmentSet
            # else we found 2
            else:
                segmentSetCipher[2] = segmentSet
        # print(f"segmentSetCipher={segmentSetCipher}")
        result = list()
        for segmentSet in cipher2Segments:
            segmentLen = len(segmentSet)
            for value, cipherSet in segmentSetCipher.items():
                if len(cipherSet) != segmentLen:
                    continue
                if len(cipherSet - segmentSet) == 0:
                    result.append(value)
                    break
        result = int("".join(map(str, result)))
        resultSum += result
    return resultSum

if __name__ == "__main__":
    main()
