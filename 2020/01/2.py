#!/usr/bin/env python3

def main():
  entries = []
  with open("input1") as f:
    for line in f:
      entries.append(int(line))
  #print(entries)
  for xi, x in enumerate(entries):
    for yi, y in enumerate(entries[xi:]):
      for z in entries[yi:]:
        if x + y + z == 2020:
          print("x: {}, y: {}, z: {}, x * y * z: {}".format(x, y, z, x * y * z))

if __name__ == '__main__':
  main()
