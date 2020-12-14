#!/usr/bin/env python3

from collections import defaultdict

def get_lines():
  lines = []
  with open("input") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def parse_line(line):
  return int(line)

def is_valid_joltage_input(joltage, adapter_rating):
  delta = adapter_rating - joltage
  return delta >= 1 and delta <= 3

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
  packs_of_ones = []
  ones_counter = 0
  for delta in deltas:
    if delta == 1:
      ones_counter = ones_counter + 1
    else:
      if ones_counter > 0:
        packs_of_ones.append(ones_counter)
      ones_counter = 0
  print("packs of ones: {}".format(packs_of_ones))
  variations = 1
  for pack_of_ones in packs_of_ones:
    if pack_of_ones == 1:
      # unnecessary but i hate this answer anyway so may as well include unnecessary ops
      variations = variations * 1
    elif pack_of_ones == 2:
      variations = variations * 2
    elif pack_of_ones == 3:
      variations = variations * 4
    elif pack_of_ones == 4:
      variations = variations * 7
  print(variations)


if __name__ == '__main__':
  main()
