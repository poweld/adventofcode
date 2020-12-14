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
    self.x = 0
    self.y = 0
    self.waypoint_x = 10
    self.waypoint_y = 1
  def move(self, direction, amount):
    print("moving {}{}".format(direction, amount))
    if direction == 'F':
      self.x = self.x + amount * self.waypoint_x
      self.y = self.y + amount * self.waypoint_y
    elif direction == 'L':
      self.move('R', 360 - amount)
    elif direction == 'R':
      amount = amount % 360
      if amount == 90:
        x_save = self.waypoint_x
        self.waypoint_x = self.waypoint_y
        self.waypoint_y = -1 * x_save
      elif amount == 180:
        self.waypoint_x = -1 * self.waypoint_x
        self.waypoint_y = -1 * self.waypoint_y
      elif amount == 270:
        x_save = self.waypoint_x
        self.waypoint_x = -1 * self.waypoint_y
        self.waypoint_y = x_save
    elif direction == 'N':
      self.waypoint_y = self.waypoint_y + amount
    elif direction == 'S':
      self.waypoint_y = self.waypoint_y - amount
    elif direction == 'E':
      self.waypoint_x = self.waypoint_x + amount
    elif direction == 'W':
      self.waypoint_x = self.waypoint_x - amount
  def manhattan_distance(self):
    return abs(self.x) + abs(self.y)
  def __str__(self):
    return "x: {}, y: {}, waypoint_x: {}, waypoint_y: {}, manhattan distance: {}".format(
      self.x,
      self.y,
      self.waypoint_x,
      self.waypoint_y,
      self.manhattan_distance()
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
