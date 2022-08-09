package main

import (
    "bufio"
    "log"
    "os"
    "strconv"
    "strings"
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
    lineLen := len(lines[0])
    var gamma string  // most common bits
    var epsilon string  // least common bits
    ones := make([]int, lineLen)
    zeroes := make([]int, lineLen)
    for _, line := range lines {
        for i, char := range line {
            switch char {
                case "1": ones[i]++
                case "0": zeroes[i]++
            }
        }
    }
}

func part2(path string) {
    log.SetPrefix("part2: ")
}
