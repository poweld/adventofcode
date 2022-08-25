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
    row, col int
}

type segment []coord

func (c coord) String() string {
    return fmt.Sprintf("{%d, %d}", c.col, c.row)
}

func abs(x int) int {
    if x < 0 {
        return -1 * x
    }
    return x
}

func (cFrom coord) segmentTo(cTo coord, ignoreDiagonals bool) segment {
    // log.Printf("making segment from %#v to %#v\n", cFrom, cTo)
    deltaRow := abs(cTo.row - cFrom.row)
    deltaCol := abs(cTo.col - cFrom.col)
    var seg segment
    var start coord
    var end coord
    if deltaCol == 0 {
        // vertical line
        if cFrom.row < cTo.row {
            start = cFrom
            end = cTo
        } else {
            start = cTo
            end = cFrom
        }
        col := cFrom.col
        seg = make(segment, 0, deltaRow)
        for row := start.row; row <= end.row; row++ {
            seg = append(seg, coord{row: row, col: col})
        }
    } else {
        if cFrom.col < cTo.col {
            start = cFrom
            end = cTo
        } else {
            start = cTo
            end = cFrom
        }
        slope := deltaRow / deltaCol
        if slope != 0 && ignoreDiagonals {
            // NOTE: ignoring diagonals
            return make(segment, 0)
        }
        // log.Printf("cFrom: %#v, cTo: %#v, slope: %+v", cFrom, cTo, slope)
        seg = make(segment, 0, deltaCol)
        //for x, y := start.x, start.y; x < end.x; x++, y += slope {
        for col, row := start.col, start.row; col <= end.col; col, row = col + 1, row + slope {
            seg = append(seg, coord{row: row, col: col})
        }
    }
    // log.Printf("returning %#v", seg)
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
    return coord{col: components[0], row: components[1]}
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

func hitsPrint(hits map[coord]int, rows int, cols int) {
    log.Printf("hitsPrint: rows: %v, cols: %v\n", rows, cols)
    s := "\n"
    for row := 0; row <= rows; row++ {
        rowDots := 0
        for col := 0; col <= cols; col++ {
            c := coord{row: row, col: col}
            count, exists := hits[c]
            if exists {
                //s += fmt.Sprintf("%d", count)
                s += strings.Repeat(".", rowDots)
                rowDots = 0
                s += strconv.Itoa(count)
            } else {
                rowDots++
            }
        }
        s += strings.Repeat(".", rowDots) + "\n"
    }
    log.Println(s)
}

func part1(path string) {
    log.SetPrefix("part1: ")
    lines, err := getLines(path)
    check(err)
    hits := make(map[coord]int)
    rows, cols := 0, 0
    for _, line := range lines {
        coord1, coord2 := parseLine(line)
        if coord1.row > rows { rows = coord1.row }
        if coord2.row > rows { rows = coord2.row }
        if coord1.col > cols { cols = coord1.col }
        if coord2.col > cols { cols = coord2.col }
        seg := coord1.segmentTo(coord2, true)
        // log.Println("seg:", seg)
        for _, c := range seg {
            _, exists := hits[c]
            if exists {
                hits[c]++
            } else {
                hits[c] = 1
            }
        }
        //hitsPrint(hits, rows, cols)
    }
    // hitsPrint(hits, rows, cols)
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
    lines, err := getLines(path)
    check(err)
    hits := make(map[coord]int)
    rows, cols := 0, 0
    for _, line := range lines {
        coord1, coord2 := parseLine(line)
        // dp-> can't figure out why, but rows and cols aren't growing as fast
        // dp-> as they should be :-/
        if coord1.row + 1 > rows { rows = coord1.row + 1 }
        if coord2.row + 1 > rows { rows = coord2.row + 1 }
        if coord1.col + 1 > cols { cols = coord1.col + 1 }
        if coord2.col + 1 > cols { cols = coord2.col + 1 }
        seg := coord1.segmentTo(coord2, false)
        for _, c := range seg {
            _, exists := hits[c]
            if exists {
                hits[c]++
            } else {
                hits[c] = 1
            }
        }
        log.Printf("seg: %#v\n", seg)
        hitsPrint(hits, rows, cols)
    }
    hitsPrint(hits, rows, cols)
    //log.Println("hits:", hits)
    overlaps := 0
    for _, val := range hits {
        if val > 1 {
            overlaps++
        }
    }
    log.Println("overlaps:", overlaps)
}
