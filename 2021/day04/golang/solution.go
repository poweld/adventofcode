package main

import (
    "bufio"
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

type Board [][]int

func (b Board) String() string {
    s := "\n"
    for _, row := range b {
        for _, col := range row {
            s += fmt.Sprintf("%2v", col) + " "
        }
        s += "\n"
    }
    return s
}

func newBoard(rows int, cols int) Board {
    board := make(Board, rows)
    for i := 0; i < rows; i++ {
        board[i] = make([]int, cols)
    }
    return board
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
    hitStrs := strings.Split(lines[0], ",")
    hits := make([]int, 0, len(hitStrs))
    for _, hitStr := range hitStrs {
        hit, err := strconv.Atoi(hitStr)
        check(err)
        //log.Println(hit)
        hits = append(hits, hit)
    }
    lines = lines[2:]
    log.Println("hits:", hits)
    //log.Println(lines)
    boards := make([]Board, 0, 100)
    board := newBoard(5, 5)
    for _, line := range lines {
        if len(line) == 0 {
            boards = append(boards, board)
            board = newBoard(5, 5)
            continue
        }
        numStrs := strings.Fields(line)
        nums := make([]int, len(numStrs))
        for i, numStr := range(numStrs) {
            num, err := strconv.Atoi(numStr)
            check(err)
            nums[i] = num
        }
        board = append(board, nums)
    }
    log.Println("test board:", boards[0])
    log.Println("board len:", len(boards[0]))
}

func part2(path string) {
    log.SetPrefix("part2: ")
}
