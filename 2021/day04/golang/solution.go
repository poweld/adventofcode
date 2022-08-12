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
        // s += fmt.Sprintf("%v: ", i)
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

type bingo interface {
    mark(row int, col int)
    won() bool
}

func (board Board) mark(n int) {
    for row := 0; row < len(board); row++ {
        for col := 0; col < len(board[row]); col++ {
            if board[row][col] == n {
                board[row][col] = -1
                return
            }
        }
    }
}

func (board Board) won() bool {
    // dp-> Should I just use a sync.WaitGroup here?
    // wg := new(sync.WaitGroup)
    // wg.Add(len(board) + len(board[0]))
    // wg.Done()
    // wg.Wait()
    winChan := make(chan bool, len(board) + len(board[0]))
    go func() {
        for row := 0; row < len(board); row++ {
            rowMarks := 0
            for col := 0; col < len(board[row]); col++ {
                if board[row][col] == -1 {
                    rowMarks++
                }
            }
            if rowMarks == len(board[0]) {
                // return true
                winChan <- true
            }
            winChan <- false
        }
    }()
    go func() {
        for col := 0; col < len(board[0]); col++ {
            colMarks := 0
            for row := 0; row < len(board); row++ {
                if board[row][col] == -1 {
                    colMarks++
                }
            }
            if colMarks == len(board) {
                // return true
                winChan <- true
            }
            winChan <- false
        }
    }()
    win := false
    for i := 0; i < len(board) + len(board[0]); i++ {
        win = win || <- winChan
    }
    return win
}

func (board Board) score(winningMark int) int {
    log.Println("scoring board:", board)
    sum := 0
    for row := 0; row < len(board); row++ {
        for col := 0; col < len(board[row]); col++ {
            if board[row][col] != -1 {
                sum += board[row][col]
            }
        }
    }
    log.Println("sum:", sum, "winningMark:", winningMark)
    return sum * winningMark
}

func main() {
    //log.SetFlags(log.Lmicroseconds)
    log.SetFlags(0)
    part1("input.txt")
    part2("input.txt")
}

func part1(path string) {
    log.SetPrefix("part1: ")
    lines, err := getLines(path)
    check(err)
    markStrs := strings.Split(lines[0], ",")
    marks := make([]int, 0, len(markStrs))
    for _, markStr := range markStrs {
        mark, err := strconv.Atoi(markStr)
        check(err)
        marks = append(marks, mark)
    }
    lines = lines[2:]
    boards := make([]Board, 0, 100)
    boardSize := 5
    board := newBoard(boardSize, boardSize)
    row := 0
    for _, line := range lines {
        if len(line) == 0 {
            boards = append(boards, board)
            board = newBoard(boardSize, boardSize)
            row = 0
            continue
        }
        numStrs := strings.Fields(line)
        nums := make([]int, len(numStrs))
        for i, numStr := range(numStrs) {
            num, err := strconv.Atoi(numStr)
            check(err)
            nums[i] = num
        }
        board[row] = nums
        row++
    }
    boards = append(boards, board)  // append the final board since there is not another blank line

    for _, mark := range marks {
        done := false
        for _, board := range boards {
            log.Println(board)
            board.mark(mark)
            if board.won() {
                log.Println("score:", board.score(mark))
                done = true
                break
            }
        }
        if done { break }
    }
}

func part2(path string) {
    log.SetPrefix("part2: ")
    lines, err := getLines(path)
    check(err)
    markStrs := strings.Split(lines[0], ",")
    marks := make([]int, 0, len(markStrs))
    for _, markStr := range markStrs {
        mark, err := strconv.Atoi(markStr)
        check(err)
        marks = append(marks, mark)
    }
    lines = lines[2:]
    boards := make([]Board, 0, 100)
    boardSize := 5
    board := newBoard(boardSize, boardSize)
    row := 0
    for _, line := range lines {
        if len(line) == 0 {
            boards = append(boards, board)
            board = newBoard(boardSize, boardSize)
            row = 0
            continue
        }
        numStrs := strings.Fields(line)
        nums := make([]int, len(numStrs))
        for i, numStr := range(numStrs) {
            num, err := strconv.Atoi(numStr)
            check(err)
            nums[i] = num
        }
        board[row] = nums
        row++
    }
    boards = append(boards, board)  // append the final board since there is not another blank line

    for _, mark := range marks {
        done := false
        for i, board := range boards {
            log.Println(board)
            board.mark(mark)
            if board.won() {
                if len(boards) == 1 {
                    log.Println("score:", board.score(mark))
                    done = true
                    break
                } else {
                    // shift elements if needed, then shorten by 1
                    if i < len(boards) - 1 {
                        copy(boards[i:], boards[i+1:])
                    }
                    boards = boards[:len(boards)-1]
                }
            }
        }
        if done { break }
    }
}
