#!/usr/bin/env python3

import functools
import operator
import re

def check_byr(value):
  return (
    len(value) == 4 and
    int(value) >= 1920 and
    int(value) <= 2002
  )

def check_iyr(value):
  return (
    len(value) == 4 and
    int(value) >= 2010 and
    int(value) <= 2020
  )

def check_eyr(value):
  return (
    len(value) == 4 and
    int(value) >= 2020 and
    int(value) <= 2030
  )

def check_hgt(value):
  metric = value[-2:]
  if metric == 'cm':
    number = int(value[:-2])
    return (
      number >= 150 and
      number <= 193
    )
  elif metric == 'in':
    number = int(value[:-2])
    return (
      number >= 59 and
      number <= 76
    )
  return False

def check_hcl(value):
  return re.search('#[a-f0-9]{6}', value)

valid_eye_colors = {
  "amb",
  "blu",
  "brn",
  "gry",
  "grn",
  "hzl",
  "oth"
}

def check_ecl(value):
  return value in valid_eye_colors

def check_pid(value):
  return len(value) == 9 and re.search('0*[0-9]*', value)

required_fields = {
  'byr': check_byr,
  'iyr': check_iyr,
  'eyr': check_eyr,
  'hgt': check_hgt,
  'hcl': check_hcl,
  'ecl': check_ecl,
  'pid': check_pid
  #'cid'
}

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

def check_key(passport, key):
  return key in passport and required_fields[key](passport[value])

def is_valid(passport):
  checks = [
    (
      field in passport and
      field_check(passport[field])
    )
    for field, field_check in required_fields.items()
  ]

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
