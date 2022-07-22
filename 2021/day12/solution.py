import string

from typing import Iterable, Tuple, List, Set, Dict
from pprint import pformat, pprint

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
    # part1TestResult = part1("testinput.txt")
    # print(f"part1 test: {part1TestResult}")
    # part1Result = part1("input.txt")
    # print(f"part1: {part1Result}")

    part2TestResult = part2("testinput.txt")
    print(f"part2 test: {part2TestResult}")
    part2Result = part2("input.txt")
    print(f"part2: {part2Result}")

def part1(filename):
    lines = getLines(filename)
    from collections import defaultdict
    connections: Dict[str, Set[str]] = defaultdict(set)
    lc = set(string.ascii_lowercase)

    def isSmall(s: str):
        return all((c in lc for c in s))

    for line in lines:
        cFrom, cTo = line.split("-")
        connections[cFrom].add(cTo)
        connections[cTo].add(cFrom)
    # print(pformat(connections))
    foundPaths: Set[str] = set()
    def paths(cFrom: str, path: List[str] = ["start"], smallVisited: Set[str] = {"start"}) -> Iterable:
        print(f"cFrom={cFrom}, path={path}, smallVisited={smallVisited}")
        if cFrom == "end":
            pathStr = ",".join(path)
            if pathStr not in foundPaths:
                foundPaths.add(pathStr)
                yield pathStr
                return
        viableConnections = filter(lambda x: x not in smallVisited, connections[cFrom])
        for connection in viableConnections:
            smallVisitedMod = set()
            if isSmall(connection):
                smallVisitedMod.add(connection)
            yield from paths(connection, path + [connection], smallVisited | smallVisitedMod)
    p = list(paths("start"))
    # pprint(p)
    return len(p)

def part2(filename):
    lines = getLines(filename)
    from collections import defaultdict
    connections: Dict[str, Set[str]] = defaultdict(set)
    lc = set(string.ascii_lowercase)

    def isSmall(s: str):
        return all((c in lc for c in s))

    for line in lines:
        cFrom, cTo = line.split("-")
        if cFrom == "start":
            connections[cFrom].add(cTo)
        elif cTo == "start":
            connections[cTo].add(cFrom)
        else:
            connections[cFrom].add(cTo)
            connections[cTo].add(cFrom)
    # print(pformat(connections))
    foundPaths: Set[str] = set()
    def paths(cFrom: str, path: List[str] = list(), smallVisited: Set[str] = set(), doubleDipped=False) -> Iterable:
        #print(f"cFrom={cFrom}, path={path}, smallVisited={smallVisited}")
        if not path:
            path = ["start"]
        if not smallVisited:
            smallVisited.add("start")
        debug = ",".join(path).startswith("start,A,c,A,b")
        if debug:
            print(f"path={','.join(path)}, smallVisited={smallVisited}, doubleDipped={doubleDipped}")
        if cFrom == "end":
            pathStr = ",".join(path)
            if pathStr not in foundPaths:
                foundPaths.add(pathStr)
                yield pathStr
                return
        viableConnections = connections[cFrom]
        if doubleDipped:
            viableConnections = list(filter(lambda x: x not in smallVisited, connections[cFrom]))
        if debug:
            print(f"viableConnections={viableConnections}")
        for connection in viableConnections:
            smallVisitedMod = set()
            if isSmall(connection):
                    smallVisitedMod.add(connection)
            newDoubleDipped = doubleDipped or (isSmall(connection) and connection in smallVisited)
            yield from paths(connection, path=path + [connection], smallVisited=smallVisited | smallVisitedMod, doubleDipped=newDoubleDipped)
    p = list(paths("start"))
    p.sort()
    print(len(p))
    for entry in p:
        print(entry)
    # pprint(p)
    return len(p)

if __name__ == "__main__":
    main()
