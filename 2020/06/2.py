#!/usr/bin/env python3

import functools
import operator

def get_lines():
  lines = []
  with open("input") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def group_lines(lines):
  group = []
  for line in lines:
    if len(line) == 0:
      yield group
      group = []
    else:
      group.append(set(line))
  yield group

def main():
  lines = get_lines()
  total_yes_answers = 0
  groups = group_lines(lines)
  #print(list(groups))
  for group in groups:
    intersection = set.intersection(*group)
    total_yes_answers = total_yes_answers + len(intersection)
    print(intersection)
  print("total yes answers: {}".format(total_yes_answers))

if __name__ == '__main__':
  main()
