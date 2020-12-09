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
      group.append(line)
  yield group

def flatten_group(group):
  flattened = []
  for person in group:
    for answer in person:
      flattened.append(answer)
  return flattened

def main():
  lines = get_lines()
  total_yes_answers = 0
  groups = group_lines(lines)
  for group in groups:
    flattened = flatten_group(group)
    yes_answers = set(flattened)
    print(yes_answers)
    print(len(yes_answers))
    total_yes_answers = total_yes_answers + len(yes_answers)
  print("total yes answers: {}".format(total_yes_answers))

if __name__ == '__main__':
  main()
