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
    horizontal := 0
    vertical := 0
    for _, line := range lines {
        parts := strings.Fields(line)
        direction := parts[0]
        amount, err := strconv.Atoi(parts[1])
        check(err)
        //log.Println(direction, amount)
        switch direction {
            case "forward": horizontal += amount
            case "down": vertical += amount
            case "up": vertical -= amount
        }
        //log.Printf("horizontal: %d, vertical: %d", horizontal, vertical)
    }
    log.Println("product:", horizontal * vertical)
}

func part2(path string) {
    log.SetPrefix("part2: ")
    lines, err := getLines(path)
    check(err)
    horizontal := 0
    vertical := 0
    aim := 0
    for _, line := range lines {
        parts := strings.Fields(line)
        direction := parts[0]
        amount, err := strconv.Atoi(parts[1])
        check(err)
        //log.Println(direction, amount)
        switch direction {
            case "forward": {
                horizontal += amount
                vertical += (aim * amount)
            }
            case "down": aim += amount
            case "up": aim -= amount
        }
        //log.Printf("horizontal: %d, vertical: %d", horizontal, vertical)
    }
    log.Println("product:", horizontal * vertical)
}
