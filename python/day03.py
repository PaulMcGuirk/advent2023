"""Advent of Code 2023 Day 3 Solver"""
import time

FILEPATH = './input/input03.txt'

def _analyze(schematic):
    size = len(schematic)

    res = 0
    gears = {}

    for r in range(size):
        digits = []
        for c in range(size):
            char = schematic[r][c]
            if char.isdigit():
                digits.append(char)
            elif digits:
                start = c - len(digits) - 1
                to_check = [(r - 1, d_c) for d_c in range(start, c + 1)]
                to_check.extend((r + 1, d_c) for d_c in range(start, c + 1))
                to_check.append((r, start))
                to_check.append((r, c))

                symbols = [(n_r, n_c, schematic[n_r][n_c] == '*')
                           for n_r, n_c in to_check
                           if schematic[n_r][n_c] != '.' and not schematic[n_r][n_c].isdigit()]

                if symbols:
                    num = int(''.join(digits))
                    res += num

                    for n_r, n_c, gear in symbols:
                        if not gear:
                            continue
                        if not (n_r, n_c) in gears:
                            gears[(n_r, n_c)] = []
                        gears[(n_r, n_c)].append(num)

                digits = []

    gear_ratio_sum = sum(nums[0]*nums[1] for nums in gears.values() if len(nums) == 2)
    return (res, gear_ratio_sum)

def _process_input(raw_input):
    padded_lines = ['.' + ln + '.' for ln in raw_input.split("\n") if ln]
    size = len(padded_lines) + 2
    padded_lines.insert(0, "." * size)
    padded_lines.append("." * size)

    return padded_lines

def _run():
    print("Advent of Code 2023")
    print("Day 3: Gear Ratios")

    tic = time.perf_counter()
    with open(FILEPATH, encoding="utf-8") as f:
        raw_input = f.read()

    processed_input = _process_input(raw_input)

    part_one, part_two = _analyze(processed_input)

    toc = time.perf_counter()

    print(f"Part one: {part_one}")
    print(f"Part two: {part_two}")

    print(f"Time elapsed: {toc - tic:04f}")

if __name__ == "__main__":
    _run()
