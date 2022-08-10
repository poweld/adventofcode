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
    lineLen := len(lines[0])
    o2 := make([]string, len(lines))  // most common bits
    copy(o2, lines)
    for i := 0; i < lineLen && len(o2) > 1; i++ {
        log.Println("o2:", o2)
        ones := 0
        zeroes := 0
        for _, line := range o2 {
            switch line[i] {
                case '1': ones++
                case '0': zeroes++
            }
        }
        var matchChar byte  // why can't we use a rune here?
        if ones >= zeroes {
            matchChar = '1'
        } else {
            matchChar = '0'
        }
        o2New := make([]string, 0, len(o2))
        for _, line := range o2 {
            if line[i] == matchChar {
                o2New = append(o2New, line)
            }
        }
        o2 = o2New
        if len(o2) == 1 { break }
    }
    log.Println("final o2:", o2)

    co2 := make([]string, len(lines))  // least common bits
    copy(co2, lines)
    for i := 0; i < lineLen && len(co2) > 1; i++ {
        log.Println("co2:", co2)
        ones := 0
        zeroes := 0
        for _, line := range co2 {
            switch line[i] {
                case '1': ones++
                case '0': zeroes++
            }
        }
        var matchChar byte  // why can't we use a rune here?
        if ones >= zeroes {
            matchChar = '0'
        } else {
            matchChar = '1'
        }
        co2New := make([]string, 0, len(co2))
        for _, line := range co2 {
            if line[i] == matchChar {
                co2New = append(co2New, line)
            }
        }
        co2 = co2New
    }
    log.Println("final co2:", co2)
    o2_i, err := strconv.ParseInt(o2[0], 2, 64)
    check(err)
    co2_i, err := strconv.ParseInt(co2[0], 2, 64)
    check(err)
    log.Println("solution:", o2_i * co2_i)
}
