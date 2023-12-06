"""Advent of Code 2023 Day 2 Solver"""
import time
import math

FILEPATH = './input/input02.txt'

COLORS = ['red', 'green', 'blue']

def _find_valid(games, targets):
    num_colors = len(COLORS)
    return sum(idx + 1 for idx, game in enumerate(games) if all(targets[i] >= game[i] for i in range(num_colors)))

def _find_power(games):
    return sum(math.prod(game) for game in games)

def _process_row(ln):
    pcs = ln.split(':')
    pulls = [[s.strip() for s in pc.strip().split(" ")] for pc in pcs[1].replace(";", ",").split(",")]
    maxes = [max(int(pull[0]) for pull in pulls if pull[1] == c) for c in COLORS]

    return maxes

def _process_input(raw_input):
    return [_process_row(ln) for ln in raw_input.split("\n") if ln]

def _run():
    print("Advent of Code 2023")
    print("Day 2: Cube Conundrum")

    tic = time.perf_counter()
    with open(FILEPATH, encoding="utf-8") as f:
        raw_input = f.read()

    processed_input = _process_input(raw_input)

    part_one = _find_valid(processed_input, [12, 13, 14])
    part_two = _find_power(processed_input)

    toc = time.perf_counter()

    print(f"Part one: {part_one}")
    print(f"Part one: {part_two}")
# 
    print(f"Time elapsed: {toc - tic:04f}")

if __name__ == "__main__":
    _run()
