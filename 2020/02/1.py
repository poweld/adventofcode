#!/usr/bin/env python3

def is_valid(password, count_min, count_max, letter) -> bool:
  match_count = 0
  for c in password:
    if c == letter:
      match_count = match_count + 1
  if match_count >= count_min and match_count <= count_max:
    return True
  return False

def main():
  lines = []
  with open("input1") as f:
    for line in f:
      lines.append(line)
  valid_count = 0
  for line in lines:
    colon_split = str.split(line, ':')
    password = colon_split[1].strip()
    space_split = str.split(colon_split[0], ' ')
    count_str = space_split[0]
    letter = space_split[1][0]
    count_min, count_max = [int(x) for x in str.split(count_str, '-')]
    #print(count_min, count_max)
    #print(letter)
    #print(password)
    if is_valid(password, count_min, count_max, letter):
      #print("{}-{} {}: {}".format(count_min, count_max, letter, password))
      valid_count = valid_count + 1
  print("valid passwords: {}".format(valid_count))


if __name__ == '__main__':
  main()
