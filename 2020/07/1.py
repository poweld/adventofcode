#!/usr/bin/env python3

def get_lines():
  lines = []
  with open("input") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def parse_line(line):
  contains_split = line.split(' bags contain ')
  container = contains_split[0]
  contents_split = contains_split[1].split(', ')
  contents = {}
  for content in contents_split:
    space_split = content.split(' ')
    quantity = 0
    if space_split[0] != 'no':
      quantity = int(space_split[0])
    else:
      continue
    bag_type = str.join(' ', space_split[1:3])
    contents[bag_type] = quantity
  parsed = { container: contents }
  return parsed

def bags_that_can_contain(bags_to_contain, bags):
  if len(bags_to_contain) == 0:
    return []
  can_contain = []
  for bag_to_contain in bags_to_contain:
    for container, contents in bags.items():
      if bag_to_contain in contents:
        can_contain.append(container)
  return can_contain + bags_that_can_contain(can_contain, bags)

def main():
  lines = get_lines()
  bags = {}
  for line in lines:
    bag = parse_line(line)
    bags = {**bags, **bag}
  can_contain_shiny_gold = bags_that_can_contain(['shiny gold'], bags)
  print(len(set(can_contain_shiny_gold)))

if __name__ == '__main__':
  main()
