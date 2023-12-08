"""Advent of Code 2023 Day 5 Solver"""
import time

FILEPATH = './input/input05.txt'

def _farm(seeds, maps, combine):
    if combine:
        intervals = [(seeds[i], seeds[i] + seeds[i + 1] - 1) for i in range(0, len(seeds), 2)]
    else:
        intervals = [(s, s) for s in seeds]

    for map_ in maps:
        new_intervals = []
        to_map = list(intervals)
        while to_map:
            start, end = to_map.pop()

            intersect = False

            for sub_map in map_:
                (dest_start, src_start, width) = sub_map
                src_end = src_start + width - 1
                intersect = start <= src_end and src_start <= end
                if not intersect:
                    continue

                sub_start = max(start, src_start)
                sub_end = min(end, src_end)

                mapped_start = dest_start + sub_start - src_start
                mapped_end = dest_start + sub_end - src_start
                new_intervals.append((mapped_start, mapped_end))

                if start < src_start:
                    to_map.append((start, src_start - 1))

                if end > src_end:
                    to_map.append((src_end + 1, end))

                break

            if not intersect:
                new_intervals.append((start, end))
        intervals = new_intervals

    return min(interval[0] for interval in intervals)


def _process_input(raw_input):
    pcs = raw_input.strip().split("\n\n")

    seeds = [int(v) for v in pcs[0].strip().split(":")[1].split(' ') if v.strip()]
    maps = [[tuple(int(v) for v in ln.split(' ') if v.strip())
             for ln in pc.split("\n")[1:] if ln.strip()] for pc in pcs[1:]]

    return seeds, maps

def _run():
    print("Advent of Code 2023")
    print("Day 5: If You Give A Seed A Fertilizer")

    tic = time.perf_counter()
    with open(FILEPATH, encoding="utf-8") as f:
        raw_input = f.read()

    seeds, maps = _process_input(raw_input)

    part_one = _farm(seeds, maps, False)
    part_two = _farm(seeds, maps, True)

    toc = time.perf_counter()

    print(f"Part one: {part_one}")
    print(f"Part two: {part_two}")

    print(f"Time elapsed: {toc - tic:04f}")

if __name__ == "__main__":
    _run()
