from itertools import pairwise
import argparse


def main(filename):
    adapters = [ int(j) for j in open(filename) ]
    print(adapters)

    chain = sorted(adapters)
    chain.insert(0, 0)
    chain.append(chain[-1] + 3)
    print(chain)

    diffs = [ i for i in pairwise(chain) ]
    diffs = [ v for v in map(lambda i: i[1] - i[0], diffs) ]
    print(diffs)

    ones = [v for v in filter(lambda v: v == 1, diffs)]
    threes = [v for v in filter(lambda v: v == 3, diffs)]
    print(len(ones) * len(threes))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("input")
    args = parser.parse_args()
    main(args.input)
