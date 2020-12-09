#!/usr/bin/env python3

def main():
  entries = []
  with open("input1") as f:
    for line in f:
      entries.append(int(line))
  #print(entries)
  for xi, x in enumerate(entries):
    for y in entries[xi:]:
      if x + y == 2020:
        print("x: {}, y: {}, x * y: {}", x, y, x * y)

if __name__ == '__main__':
  main()
