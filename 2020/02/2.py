#!/usr/bin/env python3

def is_valid(password, check_index_1, check_index_2, letter) -> bool:
  return (password[check_index_1] == letter) ^ (password[check_index_2] == letter)

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
    check_index_1, check_index_2 = [int(x) - 1 for x in str.split(count_str, '-')]
    #print(check_index_1, check_index_2)
    #print(letter)
    #print(password)
    if is_valid(password, check_index_1, check_index_2, letter):
      #print("{}-{} {}: {}".format(check_index_1, check_index_2, letter, password))
      valid_count = valid_count + 1
  print("valid passwords: {}".format(valid_count))


if __name__ == '__main__':
  main()
