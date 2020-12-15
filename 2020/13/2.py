#!/usr/bin/env python3

import itertools
import math

def get_lines():
  lines = []
  with open("input") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def parse_line(line):
  return line

class Queue:
  def __init__(self, entries=[], max_len=None):
    self.entries = entries
    self.max_len = max_len
  def __iter__(self):
    return self.entries.__iter__()
  def __str__(self):
    return self.entries.__str__()
  def __getitem__(self, x):
    return self.entries.__getitem__(x)
  def __len__(self):
    return self.entries.__len__()
  def enqueue(self, entry):
    self.entries.append(entry)
    if self.max_len is not None and len(self.entries) > self.max_len:
      return self.dequeue()
    return None
  def dequeue(self):
    return self.entries.pop(0)

def running_buses(buses):
  range_inf = itertools.count()
  for time in range_inf:
    running_buses = set()
    for bus in buses:
      if time % bus == 0:
        running_buses.add(bus)
    yield running_buses

def has_sequentially_running_buses(buses_with_xs, queue):
  if len(buses_with_xs) != len(queue):
    return False
  ret = True
  for i in range(len(buses_with_xs)):
    if buses_with_xs[i] == 'x' or int(buses_with_xs[i]) in queue[i]:
      continue
    return False
  return ret

def main():
  lines = get_lines()
  schedule = [parse_line(line) for line in lines]
  buses = set([
    int(bus)
    for bus in schedule[1].split(',')
    if bus != 'x'
  ])
  buses_with_xs = schedule[1].split(',')

  running_buses_gen = running_buses(buses)
  bus_queue = Queue(max_len=len(buses_with_xs))
  timestamp = 0
  while True:
    x = next(running_buses_gen)
    bus_queue.enqueue(x)
    if has_sequentially_running_buses(buses_with_xs, bus_queue):
      break
    #if timestamp >= 1_068_781:
      #print("ts: {}, queue: {}, buses_with_xs: {}".format(timestamp, bus_queue, buses_with_xs))
    timestamp = timestamp + 1
  print("done: ts: {}, queue: {}, buses_with_xs: {}".format(timestamp, bus_queue, buses_with_xs))


if __name__ == '__main__':
  main()
