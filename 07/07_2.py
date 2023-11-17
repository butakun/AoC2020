from pprint import pprint


def main(filename):
    with open(filename) as f:
        bags = {}
        for line in f:
            line = line.strip()
            line = line.replace("contains", "contain")
            line = line.replace("bags", "bag")

            outer, inners = line.split("contain")
            tokens = outer.strip().split()
            assert(tokens[-1] == "bag")
            outer = " ".join(tokens[0:-1])

            inners = inners.strip(".").split(",")
            inners = [i.strip() for i in inners]
            contains = {}
            for inner in inners:
                tokens = inner.split()
                if tokens[0] == "no":
                    num = 0
                    assert tokens[1] == "other" and tokens[-1] == "bag"
                else:
                    num = int(tokens[0])
                    assert tokens[-1] == "bag"
                    color = " ".join(tokens[1:-1])
                    assert color not in contains
                    contains[color] = num
            assert outer not in bags
            bags[outer] = contains

    pprint(bags)

    def dfs(color):
        print(f"** {color} contains")
        num_contains = 0
        for inner, num in bags[color].items():
            print(inner, num)
            num_contains += num * dfs(inner)
        return num_contains + 1

    num_bags = dfs("shiny gold")
    print(num_bags - 1)


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("input")
    args = parser.parse_args()
    main(args.input)
