from itertools import pairwise
import argparse


def build(node, adapters, table):
    if node == adapters[-1]:
        return 1

    children = [j for j in adapters if node < j and j <= node + 3]
    count_children = 0
    for child in children:
        if child in table:
            print(f"cache hit {child}")
            count_children += table[child]
        else:
            count_children += build(child, adapters, table)
    table[node] = count_children

    return count_children


def main(filename):
    adapters = sorted([ int(j) for j in open(filename) ])
    adapters.append(adapters[-1] + 3)
    print(adapters)
    last = adapters[-1]

    table = {}
    count = build(0, adapters, table)
    print(table)
    print(count)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("input", nargs="?", default="input.txt")
    args = parser.parse_args()
    main(args.input)
