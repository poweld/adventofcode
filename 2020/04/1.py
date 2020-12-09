#!/usr/bin/env python3

import functools
import operator

required_fields = [
  'byr',
  'iyr',
  'eyr',
  'hgt',
  'hcl',
  'ecl',
  'pid'
  #'cid'
]

def to_passports(lines):
  acc = {}
  for line in lines:
    if len(line) == 0:
      yield acc
      acc = {}
    else:
      kvs = str.split(line, ' ')
      if len(kvs) == 0:
        kvs = [line]
      for kv in kvs:
        k, v = str.split(kv, ':')
        acc[k] = v
  yield acc

def is_valid(passport):
  checks = [required_field in passport for required_field in required_fields]
  return all(checks)

def main():
  lines = []
  with open("input1") as f:
    for line in f:
      lines.append(line.strip())
  valid_passports = 0
  invalid_passports = 0
  for passport in to_passports(lines):
    print(passport)
    print("field count: {}".format(len(passport.keys())))
    is_valid_passport = is_valid(passport)
    print(is_valid_passport)
    if is_valid_passport:
      valid_passports = valid_passports + 1
    else:
      invalid_passports = invalid_passports + 1
  print("valid passports: {}".format(valid_passports))
  print("invalid passports: {}".format(invalid_passports))

if __name__ == '__main__':
  main()
