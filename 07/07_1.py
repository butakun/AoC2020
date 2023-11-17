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

    graph = {}

    for outer, contains in bags.items():
        for color, num in contains.items():
            if color in graph:
                graph[color].add(outer)
            else:
                graph[color] = set([outer])

    pprint(graph)

    discovered = set()
    def dfs(color):
        discovered.add(color)
        if color not in graph:
            return
        for outer in graph[color]:
            if outer not in discovered:
                dfs(outer)

    dfs("shiny gold")
    discovered.remove("shiny gold")
    print(discovered)
    print(len(discovered))


if __name__ == "__main__":
    import sys
    main(sys.argv[1])
