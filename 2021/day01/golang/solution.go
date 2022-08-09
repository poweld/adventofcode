package main

import (
    "bufio"
    //"container/list"
    "log"
    "os"
    "strconv"
)


func check(err error) {
    if err != nil {
        log.Fatal(err)
    }
}

func getLines(path string) ([]string, error) {
    file, err := os.Open(path)
    defer file.Close()

    if err != nil {
        return nil, err
    }

    scanner := bufio.NewScanner(file)
    lines := make([]string, 0, 3000)
    for scanner.Scan() {
        line := scanner.Text()
        lines = append(lines, line)
    }

    return lines, nil
}

func main() {
    log.SetFlags(log.Lmicroseconds)
    part1("input.txt")
    part2("input.txt")
}

func part1(path string) {
    log.SetPrefix("part1: ")
    lines, err := getLines(path)
    check(err)

    nums := make([]int, 0, len(lines))
    for _, line := range lines {
        num, err := strconv.Atoi(line)
        check(err)
        nums = append(nums, num)
    }
    var prev *int = nil
    increases := 0
    for i, num := range nums {
        if prev != nil && *prev < num {
            increases++
        }
        prev = &nums[i]
    }
    log.Printf("increases: %d", increases)
}

func part2(path string) {
    log.SetPrefix("part2: ")
    lines, err := getLines(path)
    check(err)

    nums := make([]int, 0, len(lines))
    for _, line := range lines {
        num, err := strconv.Atoi(line)
        check(err)
        nums = append(nums, num)
    }
    increases := 0
    for i, _ := range nums {
        if i > len(nums) - 4 { break }
        window := nums[i:i + 4]
        if len(window) < 4 { break }
        if window[0] < window[3] { increases++ }
    }
    log.Printf("increases: %d", increases)
}
