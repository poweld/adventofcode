#!/usr/bin/env python3

def get_lines():
  lines = []
  with open("input") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def parse_line(line):
  return int(line)

class Queue:
  def __init__(self, entries=[]):
    self.entries = entries
  def __iter__(self):
    return self.entries.__iter__()
  def __str__(self):
    return self.entries.__str__()
  def __getitem__(self, x):
    return self.entries.__getitem__(x)
  def put(self, entry):
    self.entries.append(entry)
  def get(self):
    return self.entries.pop(0)

def entry_in_previous_sums(previous_entries, entry):
  sums = []
  for xi, x in enumerate(previous_entries):
    for y in previous_entries[xi:]:
      sums.append(x + y)
  sums_set = set(sums)
  return entry in sums

def contiguous_sum(entries, entry):
  for xi in range(len(entries)):
    for yi in range(xi + 1, len(entries)):
      if sum(entries[xi:yi]) == entry:
        return entries[xi:yi]

def main():
  lines = get_lines()
  entries = [parse_line(line) for line in lines]
  preamble_length = 25
  previous_entries = Queue(entries[:preamble_length])
  entries = entries[preamble_length:]
  invalid_entry = None
  for entry in entries:
    if not entry_in_previous_sums(previous_entries, entry):
      print("{} not a sum of previous {} entries: {}".format(entry, preamble_length, previous_entries))
      invalid_entry = entry
      break
    previous_entries.get()
    previous_entries.put(entry)
  contiguous_sum_entries = contiguous_sum(entries, invalid_entry)
  print("{} is the sum of contiguous entries: {}".format(invalid_entry, contiguous_sum_entries))
  mi, ma = [min(contiguous_sum_entries), max(contiguous_sum_entries)]
  print("min: {}, max: {}, sum: {}".format(mi, ma, mi + ma))


if __name__ == '__main__':
  main()
