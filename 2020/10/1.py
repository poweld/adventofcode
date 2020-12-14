#!/usr/bin/env python3

from collections import defaultdict

def get_lines():
  lines = []
  with open("test_input") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def parse_line(line):
  return int(line)

def is_valid_joltage_input(joltage, adapter_rating):
  delta = adapter_rating - joltage
  return delta >= 1 and delta <= 3

def device_max_joltage_rating(adapter_ratings):
  return max(ratings) + 3

def main():
  lines = get_lines()
  adapter_ratings = [parse_line(line) for line in lines]
  adapter_ratings.insert(0, 0) # wall joltage
  adapter_ratings.append(max(adapter_ratings) + 3) # device joltage
  adapter_ratings.sort()
  print(adapter_ratings)
  joltage_deltas = defaultdict(lambda: 0)
  for i in range(len(adapter_ratings) - 1):
    delta = adapter_ratings[i + 1] - adapter_ratings[i]
    joltage_deltas[delta] = joltage_deltas[delta] + 1
  print("1-jolt deltas: {}, 3-jolt deltas: {}, product: {}".format(
    joltage_deltas[1],
    joltage_deltas[3],
    joltage_deltas[1] * joltage_deltas[3]
  ))


if __name__ == '__main__':
  main()
