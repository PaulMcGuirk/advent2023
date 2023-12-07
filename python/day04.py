"""Advent of Code 2023 Day 4 Solver"""
import time

FILEPATH = './input/input04.txt'

def _count_matches(winners, drawn):
    return len(winners & drawn)

def _score(win_count):
    return 1 << win_count >> 1

def _scratchcards(cards):
    return sum(_score(_count_matches(winners, drawn)) for winners, drawn in cards)

def _scratchcards_elvish(cards):
    tots = [1] * len(cards)

    for i, game in enumerate(cards):
        winners, drawn = game
        wins = _count_matches(winners, drawn)
        end = min(i + wins + 1, len(cards))
        for j in range(i + 1, end):
            tots[j] += tots[i]

    return sum(tots)

def _process_input(raw_input):
    return [tuple({int(v.strip()) for v in pc.split(' ') if v.strip()} for pc in ln.split(":")[1].split('|'))
            for ln in raw_input.strip().split('\n')]

def _run():
    print("Advent of Code 2023")
    print("Day 4: Scratchcards")

    tic = time.perf_counter()
    with open(FILEPATH, encoding="utf-8") as f:
        raw_input = f.read()

    processed_input = _process_input(raw_input)

    part_one = _scratchcards(processed_input)
    part_two = _scratchcards_elvish(processed_input)

    toc = time.perf_counter()

    print(f"Part one: {part_one}")
    print(f"Part two: {part_two}")

    print(f"Time elapsed: {toc - tic:04f}")

if __name__ == "__main__":
    _run()
