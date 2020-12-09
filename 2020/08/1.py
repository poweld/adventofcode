#!/usr/bin/env python3

def get_lines():
  lines = []
  with open("input") as f:
    for line in f:
      lines.append(line.strip())
  return lines

def parse_line(line):
  space_split = line.split(' ')
  operation = space_split[0]
  argument = int(space_split[1])
  return { 'operation': operation, 'argument': argument }

def handle_instruction(instruction_pointer, instructions, acc=0, seen_ips=set()):
  if instruction_pointer in seen_ips:
    return acc
  new_seen_ips = seen_ips.union({instruction_pointer})
  operation = instructions[instruction_pointer]['operation']
  argument = instructions[instruction_pointer]['argument']
  if operation == 'nop':
    return handle_instruction(instruction_pointer + 1, instructions, acc, new_seen_ips)
  elif operation == 'jmp':
    return handle_instruction(instruction_pointer + argument, instructions, acc, new_seen_ips)
  elif operation == 'acc':
    return handle_instruction(instruction_pointer + 1, instructions, acc + argument, new_seen_ips)


def main():
  lines = get_lines()
  instructions = [parse_line(line) for line in lines]
  acc = handle_instruction(0, instructions)
  print(acc)


if __name__ == '__main__':
  main()
