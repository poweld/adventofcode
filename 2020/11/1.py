#!/usr/bin/env python3

from collections import defaultdict
import copy

def get_lines():
  lines = []
  with open("input") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def parse_line(line):
  return list(line)

class Board:
  def __init__(self, board):
    self.board = board
  def __str__(self):
    xs = ["".join(self.board[y]) for y in range(len(self.board))]
    return "\n".join(xs)
  def value_at(self, x, y):
    return self.board[y][x]
  def set_occupied(self, x, y):
    self.board[y][x] = '#'
  def set_empty(self, x, y):
    self.board[y][x] = 'L'
  def occupied_seat_count(self):
    occupied_count = 0
    for y in range(len(self.board)):
      for x in range(len(self.board[y])):
        if self.is_occupied(x, y):
          occupied_count = occupied_count + 1
    return occupied_count
  def is_valid_location(self, x, y):
    return (
      y >= 0 and y < len(self.board) and
      x >= 0 and
      x < len(self.board[y])
    )
  def is_occupied(self, x, y):
    return (
      self.is_valid_location(x, y) and
      self.value_at(x, y) == '#'
    )
  def is_empty(self, x, y):
    return (
      self.is_valid_location(x, y) and
      self.value_at(x, y) == 'L'
    )
  def neighboring_occupied(self, x, y):
    occupied = 0
    for xn in range(x - 1, x + 2):
      for yn in range(y - 1, y + 2):
        if xn == x and yn == y:
          continue
        if self.is_occupied(xn, yn):
          occupied = occupied + 1
    return occupied
  def step(self):
    # alters the board, returns whether stable or not
    _b = copy.deepcopy(self.board)
    new_board = Board(_b)
    stable = True
    for y in range(len(self.board)):
      for x in range(len(self.board[y])):
        if self.is_empty(x, y):
          if self.neighboring_occupied(x, y) == 0:
            new_board.set_occupied(x, y)
            stable = False
        if self.is_occupied(x, y):
          if self.neighboring_occupied(x, y) >= 4:
            new_board.set_empty(x, y)
            stable = False
    self.board = new_board.board
    return stable

def main():
  lines = get_lines()
  seat_layout = [parse_line(line) for line in lines]
  board = Board(seat_layout)
  # print(str(board) + "\n")
  # board.step()
  steps = 0
  while not board.step():
    steps = steps + 1
  print("final board: \n{}".format(str(board)))
  print("steps until stable: {}".format(steps))
  print("final occupied count: {}".format(board.occupied_seat_count()))
  board.step()
  #print("check stable board: \n{}".format(str(board)))
if __name__ == '__main__':
  main()
