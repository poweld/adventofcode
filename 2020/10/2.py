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

#def device_max_joltage_rating(adapter_ratings):
#  return max(ratings) + 3
#
#def variation_to_str(variation):
#  return ','.join([str(joltage) for joltage in variation])
#
#def get_valid_variations(adapter_ratings, start_at_index=1, seen_variation_strs=set()):
#  variation_str = variation_to_str(adapter_ratings)
#  print("len: {}".format(len(adapter_ratings)))
#  print("starting at {}".format(start_at_index))
#  if variation_str in seen_variation_strs:
#    print("skip")
#    return []
#  seen_variation_strs.add(variation_str)
#  valid_variations = [adapter_ratings]
#  for i in range(start_at_index, len(adapter_ratings) - 3):
#    if (is_valid_joltage_input(adapter_ratings[i], adapter_ratings[i + 2])):
#      # can skip over the (i + 1) index
#      new_adapters = adapter_ratings.copy()
#      del new_adapters[i + 1]
#      print("{}: found jump".format(adapter_ratings))
#      valid_variations.extend(get_valid_variations(new_adapters, i, seen_variation_strs))
#  print("done")
#  return valid_variations

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
  skippable = set()
  num_skips = 0
  for i in range(len(adapter_ratings) - 3):
    if is_valid_joltage_input(adapter_ratings[i], adapter_ratings[i + 2]):
      skippable.add(adapter_ratings[i + 1])
      num_skips = num_skips + 1
    else:
      continue
    if is_valid_joltage_input(adapter_ratings[i], adapter_ratings[i + 3]):
      skippable.add(adapter_ratings[i + 2])
      num_skips = num_skips + 1
  print("jumps found: {}, 2 ** jumps_found: {}".format(len(skippable), 2 ** len(skippable)))
  print("num skips found: {}".format(num_skips))
  deltas = to_deltas(adapter_ratings)
  print("deltas: {}".format(deltas))
  deltas_till_3_stack = []
  deltas_till_3 = 0
  for delta in deltas:
    if deltas_till_3 + delta <= 3:
      deltas_till_3 = deltas_till_3 + 1
    else:
      if deltas_till_3 > 1:
        deltas_till_3_stack.append(deltas_till_3)
      deltas_till_3 = 0
  print(deltas_till_3_stack)
  variations_mul = 1
  variations_add = 0
  variations = 1
  for _ in range(len(deltas_till_3_stack)):
    delta = deltas_till_3_stack.pop()
    print("delta: {}".format(delta))
    variations = variations + 1
    variations_mul = variations_mul * delta
    variations_add = variations_add + 1
  print(variations_mul + variations_add)
  #valid_variations = get_valid_variations(adapter_ratings)
  #valid_variation_strings = [str.join(',', [str(vx) for vx in v]) for v in valid_variations]
  #unique_variations = set(valid_variation_strings)
  #for v in unique_variations:
  #  print(v)
  #print("valid variation count: {}".format(len(unique_variations)))

if __name__ == '__main__':
  main()
