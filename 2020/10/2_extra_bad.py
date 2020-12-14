#!/usr/bin/env python3

from collections import defaultdict

def get_lines():
  lines = []
  with open("input") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def stupid_get_answer(deltas, ones=0, acc=1):
  print("deltas: {}, ones: {}, acc: {}".format(deltas, ones, acc))
  if len(deltas) == 0:
    return acc
  if deltas[0] == 1:
    return stupid_get_answer(deltas[1:], ones=ones + 1, acc=acc)
  else:
    if ones == 2:
      return stupid_get_answer(deltas[1:], acc=2 * acc)
    elif ones == 3:
      return stupid_get_answer(deltas[1:], acc=4 * acc)
    elif ones == 4:
      return stupid_get_answer(deltas[1:], acc=7 * acc)
    return stupid_get_answer(deltas[1:], acc=acc)

def parse_line(line):
  return int(line)

def to_deltas(adapter_ratings):
  deltas = []
  for i in range(len(adapter_ratings) - 1):
    deltas.append(adapter_ratings[i + 1] - adapter_ratings[i])
  return deltas

def main():
  lines = get_lines()
  adapter_ratings = [parse_line(line) for line in lines]
  adapter_ratings.insert(0, 0) # wall joltage
  adapter_ratings.append(max(adapter_ratings) + 3) # device joltage
  adapter_ratings.sort()
  print(adapter_ratings)
  deltas = to_deltas(adapter_ratings)
  print("deltas: {}".format(deltas))
  print("stupid answer: {}".format(stupid_get_answer(deltas)))


if __name__ == '__main__':
  main()
