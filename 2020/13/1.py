#!/usr/bin/env python3

import itertools

def get_lines():
  lines = []
  with open("input") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def parse_line(line):
  return line

def running_buses(buses):
  range_inf = itertools.count()
  for time in range_inf:
    running_buses = set()
    for bus in buses:
      if time % bus == 0:
        running_buses.add(bus)
    yield running_buses

def main():
  lines = get_lines()
  schedule = [parse_line(line) for line in lines]
  earliest_departure = int(schedule[0])
  buses = set([
    int(bus)
    for bus in schedule[1].split(',')
    if bus != 'x'
  ])
  print(buses)
  running_buses_gen = running_buses(buses)
  for x in range(earliest_departure):
    print("{}: {}".format(x, next(running_buses_gen)))
  print("=== at earliest departure time ===")
  departure_time = earliest_departure
  chosen_bus = None
  while True:
    x = next(running_buses_gen)
    print("{}: {}".format(departure_time, x))
    if len(x) == 0:
      departure_time = departure_time + 1
    else:
      chosen_bus = min(x)
      break
  waited = departure_time - earliest_departure
  print("taking bus {} at time {}, waited {}".format(chosen_bus, departure_time, waited))
  print("bus ID * minutes waited: {}".format(waited * chosen_bus))


if __name__ == '__main__':
  main()
