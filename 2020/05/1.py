#!/usr/bin/env python3

import functools
import operator

def get_lines():
  lines = []
  with open("input1") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def main():
  lines = get_lines()
  max_seat_id = 0
  for line in lines:
    row_chars = line[:7]
    row = 0
    for i in range(len(row_chars)):
      row_char = row_chars[-1 * (i + 1)]
      if row_char == 'B':
        row = row + (2 ** i)
        #print('B')
      #else:
      #  print('F')

    col_chars = line[7:]
    col = 0
    for i in range(len(col_chars)):
      col_char = col_chars[-1 * (i + 1)]
      if col_char == 'R':
        col = col + (2 ** i)
        #print('R')
      #else:
      #  print('L')

    seat_id = (row * 8) + col
    if max_seat_id < seat_id:
      max_seat_id = seat_id
    print("row: {}, col: {}, seat_id: {}".format(row, col, seat_id))
  print("max_seat_id: {}".format(max_seat_id))

if __name__ == '__main__':
  main()
