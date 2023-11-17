import argparse


def main(filename):
    with open(filename) as f:
        insts = [line.strip().split() for line in f]

    insts = [[l[0], int(l[1])] for l in insts]
    executed = [False] * len(insts)

    acc = 0
    i = 0
    while i < len(insts):
        if executed[i]:
            break
        cmd, count = insts[i][0], insts[i][1]
        executed[i] = True
        if cmd == "acc":
            acc += count
            i += 1
        elif cmd == "jmp":
            i += count
        elif cmd == "nop":
            i += 1
        else:
            raise ValueError(f"{insts[i]}")
        print(i, acc)

    print("final counter = ", acc)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("input")
    args = parser.parse_args()
    main(args.input)
