from typing import Iterable

def getLines(filename="input.txt") -> Iterable[str]:
    with open(filename) as f:
        line = f.readline()
        while line:
            yield line[:-1]
            line = f.readline()

def main():
    print(f"part1: {part1()}")
    print(f"part2: {part2()}")

def part1():
    lines = getLines()
    nums = map(lambda x: int(x), lines)
    prev = next(nums)
    increases = 0
    for num in nums:
        # print(f"num={num}, prev={prev}")
        if num > prev:
            increases += 1
        prev = num
    return f"increases={increases}"

def part2():
    lines = getLines()
    nums = map(lambda x: int(x), lines)

    # maintain a 3 number (temporarily 4) window and just compare the first and fourth to determine increase
    from collections import deque
    window = deque()
    while len(window) < 3:
        window.append(next(nums))
    increases = 0
    for num in nums:
        if window.popleft() < num:
            increases += 1
        window.append(num)
    return f"increases={increases}"

if __name__ == "__main__":
    main()
