package main

import (
    "bufio"
    "flag"
    "fmt"
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

type coord struct {
    x, y int
}

type segment []coord

func (c coord) String() string {
    return fmt.Sprintf("{%d, %d}", c.x, c.y)
}

func abs(x int) int {
    if x < 0 {
        return -1 * x
    }
    return x
}

func (cFrom coord) segmentTo(cTo coord) segment {
    deltaY := abs(cTo.y - cFrom.y)
    deltaX := abs(cTo.x - cFrom.x)
    var seg segment
    var start coord
    var end coord
    if deltaX == 0 {
        // vertical line
        if cFrom.y < cTo.y {
            start = cFrom
            end = cTo
        } else {
            start = cTo
            end = cFrom
        }
        x := cFrom.x
        seg := make(segment, 0, deltaY)
        for y := start.y; y < end.y; y++ {
            seg = append(seg, coord{x, y})
        }
    } else {
        if cFrom.x < cTo.x {
            start = cFrom
            end = cTo
        } else {
            start = cTo
            end = cFrom
        }
        slope := deltaY / deltaX
        seg = make(segment, 0, deltaX)
        //for x, y := start.x, start.y; x < end.x; x++, y += slope {
        for x, y := start.x, start.y; x < end.x; x, y = x + 1, y + slope {
            seg = append(seg, coord{x, y})
        }
    }
    return seg
}

func parseLine(line string) (coord, coord) {
    coordStrs := strings.Split(line, " -> ")
    coord1 := makeCoord(coordStrs[0])
    coord2 := makeCoord(coordStrs[1])
    return coord1, coord2
}

func makeCoord(coordStr string) coord {
    parts := strings.Split(coordStr, ",")
    components := make([]int, 2)
    for i, part := range parts {
        component, err := strconv.Atoi(part)
        check(err)
        components[i] = component
    }
    return coord{components[0], components[1]}
}

func help() {
    log.Fatal("Usage: solution [-part1] [-part2] [-test] [-h]")
}


func main() {
    //log.SetFlags(log.Lmicroseconds)
    log.SetFlags(0)

    pPart1 := flag.Bool("part1", false, "run only part 1")
    pPart2 := flag.Bool("part2", false, "run only part 2")
    pTest := flag.Bool("test", false, "run against testinput.txt")
    pHelp := flag.Bool("help", false, "show help text")
    flag.Parse()

    if *pHelp {
        help()
    }

    var inputfile string
    if *pTest {
        inputfile = "testinput.txt"
    } else {
        inputfile = "input.txt"
    }

    if *pPart1 {
        part1(inputfile)
    } else if *pPart2 {
        part2(inputfile)
    } else {
        part1(inputfile)
        part2(inputfile)
    }
}

func part1(path string) {
    log.SetPrefix("part1: ")
    lines, err := getLines(path)
    check(err)
    hits := make(map[coord]int)
    for _, line := range lines {
        coord1, coord2 := parseLine(line)
        seg := coord1.segmentTo(coord2)
        for _, c := range seg {
            _, exists := hits[c]
            if exists {
                hits[c]++
            } else {
                hits[c] = 1
            }
        }
    }
    //log.Println("hits:", hits)
    overlaps := 0
    for _, val := range hits {
        if val > 1 {
            overlaps++
        }
    }
    log.Println("overlaps:", overlaps)
    // dp-> think i mixed up x and y (row and col?)
}

func part2(path string) {
    log.SetPrefix("part2: ")
}
