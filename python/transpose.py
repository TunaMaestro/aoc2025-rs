with open("/dev/stdin") as inp:
    lines = inp.readlines()

transposed = zip(*lines)

print("\n".join(["".join(x) for x in transposed]))


