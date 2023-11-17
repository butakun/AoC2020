from copy import deepcopy

def main(filename):
    with open(filename) as f:
        insts = [line.strip().split() for line in f]

    insts = [[l[0], int(l[1])] for l in insts]

    cmds_to_mod = [i for i, cmd in enumerate(insts) if cmd[0] == "nop" or cmd[0] == "jmp"]
    for i in cmds_to_mod:
        print(i, insts[i])
        insts2 = deepcopy(insts)
        if insts2[i][0] == "jmp":
            insts2[i][0] = "nop"
        elif insts2[i][0] == "nop":
            insts2[i][0] = "jmp"
        else:
            raise ValueError

        status, acc = run(insts2)
        if status:
            print(status, acc)
            break


def run(insts):
    executed = [False] * len(insts)
    acc = 0
    i = 0
    while i < len(insts):
        if executed[i]:
            return False, acc

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

    return True, acc


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("input")
    args = parser.parse_args()
    main(args.input)
