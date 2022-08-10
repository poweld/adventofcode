package main

import (
    "bufio"
    "log"
    "os"
    "strconv"
    // "strings"
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
                case '1': ones[i]++
                case '0': zeroes[i]++
            }
        }
    }
    log.Println("\tones:\t", ones)
    log.Println("\tzeroes:\t", zeroes)
    for i, _ := range ones {
        if ones[i] > zeroes[i] {
            gamma += "1"
            epsilon += "0"
        } else {
            gamma += "0"
            epsilon += "1"
        }
    }
    log.Println(gamma, epsilon)
    gamma_i, err := strconv.ParseInt(gamma, 2, 64)
    check(err)
    epsilon_i, err := strconv.ParseInt(epsilon, 2, 64)
    check(err)
    log.Println(gamma_i, epsilon_i)
    solution := gamma_i * epsilon_i
    log.Println("solution:", solution)
}

func part2(path string) {
    log.SetPrefix("part2: ")
    lines, err := getLines(path)
    check(err)
    // lineLen := len(lines[0])

    var getO2 func([]string, int) string
    getO2 = func(lines []string, index int) string {
        if len(lines) == 1 {
            return lines[0]
        }
        zeroes := make([]string, 0, len(lines))
        ones := make([]string, 0, len(lines))
        for _, line := range lines {
            if line[index] == '0' {
                zeroes = append(zeroes, line)
            } else {
                ones = append(ones, line)
            }
        }
        if len(ones) >= len(zeroes) {
            return getO2(ones, index + 1)
        }
        return getO2(zeroes, index + 1)
    }

    var getCO2 func([]string, int) string
    getCO2 = func(lines []string, index int) string {
        if len(lines) == 1 {
            return lines[0]
        }
        zeroes := make([]string, 0, len(lines))
        ones := make([]string, 0, len(lines))
        for _, line := range lines {
            if line[index] == '0' {
                zeroes = append(zeroes, line)
            } else {
                ones = append(ones, line)
            }
        }
        if len(zeroes) <= len(ones) {
            return getCO2(zeroes, index + 1)
        }
        return getCO2(ones, index + 1)
    }

    o2 := getO2(lines, 0)
    co2 := getCO2(lines, 0)
    o2_i, err := strconv.ParseInt(o2, 2, 64)
    check(err)
    co2_i, err := strconv.ParseInt(co2, 2, 64)
    check(err)
    log.Println("solution:", o2_i * co2_i)
}
