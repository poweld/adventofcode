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

def bags_within(bags, all_bags):
  if len(bags) == 0:
    return 0
  within_bags = []
  for bag in bags:
    contents = all_bags[bag]
    for b, quantity in contents.items():
      within_bags = within_bags + [b for i in range(quantity)]
  print("bags within {}: {}".format(bags, within_bags))
  return len(within_bags) + bags_within(within_bags, all_bags)

def main():
  lines = get_lines()
  bags = {}
  for line in lines:
    bag = parse_line(line)
    bags = {**bags, **bag}
  bags_within_shiny_gold = bags_within(['shiny gold'], bags)
  print(bags_within_shiny_gold)

if __name__ == '__main__':
  main()
