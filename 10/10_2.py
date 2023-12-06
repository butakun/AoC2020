from itertools import pairwise
import argparse


def build(node, adapters, table):
    jolt, children = node

    if jolt == adapters[-1]:
        return 1

    children = [[j, None] for j in adapters if jolt < j and j <= jolt + 3]
    count_children = 0
    for child in children:
        child_jolt = child[0]
        if child_jolt in table:
            print(f"cache hit {child_jolt}")
            count_children += table[child_jolt]
        else:
            count_children += build(child, adapters, table)
    table[jolt] = count_children

    node[1] = children
    return count_children


def main(filename):
    adapters = sorted([ int(j) for j in open(filename) ])
    print(adapters)

    adapters.append(adapters[-1] + 3)
    print(adapters)
    last = adapters[-1]

    root = [0, None]
    table = {}
    count = build(root, adapters, table)
    print(table)
    print(count)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("input", default="input.txt")
    args = parser.parse_args()
    main(args.input)
