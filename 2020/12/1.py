#!/usr/bin/env python3

from enum import Enum
import math
import re

def get_lines():
  lines = []
  with open("input") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def parse_line(line):
  parsed = re.match(r'([NSEWLRF])(\d+)', line)
  return [parsed.group(1), int(parsed.group(2))]

class Boat:
  def __init__(self):
    self.bearing = 90  # begin facing east
    self.x = 0
    self.y = 0
  def move(self, direction, amount):
    print("moving {}{}".format(direction, amount))
    if direction == 'F':
      if self.bearing == 0:  # north
        self.move('N', amount)
      elif self.bearing == 90:  # east
        self.move('E', amount)
      elif self.bearing == 180:  # south
        self.move('S', amount)
      elif self.bearing == 270:  # east
        self.move('W', amount)
    elif direction == 'L':
      self.bearing = (self.bearing - amount) % 360
    elif direction == 'R':
      self.bearing = (self.bearing + amount) % 360
    elif direction == 'N':
      self.y = self.y + amount
    elif direction == 'S':
      self.y = self.y - amount
    elif direction == 'E':
      self.x = self.x + amount
    elif direction == 'W':
      self.x = self.x - amount
  def manhattan_distance(self):
    return abs(self.x) + abs(self.y)
  def __str__(self):
    return "x: {}, y: {}, bearing: {}, manhattan distance: {}".format(
      self.x, self.y, self.bearing, self.manhattan_distance()
    )

def main():
  lines = get_lines()
  moves = [parse_line(line) for line in lines]
  print(moves)
  print()
  boat = Boat()
  for move in moves:
    direction, amount = move
    boat.move(direction, amount)
    print(boat)

if __name__ == '__main__':
  main()
