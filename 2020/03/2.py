#!/usr/bin/env python3

import functools
import operator

def main():
  lines = []
  with open("input1") as f:
    for line in f:
      lines.append(line)
  slopes = [
    [1, -1],
    [3, -1],
    [5, -1],
    [7, -1],
    [1, -2]
  ]
  tree_counts = []
  for slope in slopes:
    slope_x, slope_y = slope
    tree_count = 0
    x = 0
    y = 0
    for line in [l.strip() for l in lines]:
      if y % slope_y == 0:
        line_len = len(line)
        xi = x % line_len
        print(line[:xi], end="")
        if line[xi] == '#':
          tree_count = tree_count + 1
          print('X', end="")
        else:
          print('O', end="")
        print(line[xi:])
      else:
        print(line)
      x = x + slope_x
      y = y + 1
    print(tree_count)
    tree_counts.append(tree_count)
  tree_product = functools.reduce(operator.mul, tree_counts)
  print("tree counts: {}".format(tree_counts))
  print("tree product: {}".format(tree_product))

if __name__ == '__main__':
  main()
