"""Advent of Code 2023 Day 1 Solver"""
import time

FILEPATH = './input/input01.txt'

DIGITS = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',\
          'zero', 'one', 'two', 'three', 'four',
          'five', 'six', 'seven', 'eight', 'nine']

def _calibrate(ln, targets):
    lefts = ((ln.find(s), idx % 10) for idx, s in enumerate(targets))
    tens = min((p for p in lefts if p[0] >= 0), key=lambda p: p[0])[1]

    rights = ((ln.rfind(s) - len(s) + 1, idx % 10) for idx, s in enumerate(targets))
    ones = max((p for p in rights), key=lambda p: p[0])[1]

    return 10 * tens + ones

def _solve(raw_input, spell):
    targets = DIGITS if spell else DIGITS[:10]
    return sum(_calibrate(ln, targets) for ln in raw_input.split('\n') if ln)

def _run():
    print("Advent of Code 2023")
    print("Day 1: Trebuchet?!")

    tic = time.perf_counter()
    with open(FILEPATH, encoding="utf-8") as f:
        raw_input = f.read()

    part_one = _solve(raw_input, False)
    part_two = _solve(raw_input, True)

    toc = time.perf_counter()

    print(f"Part one: {part_one}")
    print(f"Part one: {part_two}")

    print(f"Time elapsed: {toc - tic:04f}")

if __name__ == "__main__":
    _run()
