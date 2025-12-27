def parse(filename: str) -> list[tuple[list[list[int]], list[int]]]:
    input = []
    with open(filename) as f:
        for line in f.readlines():
            parts = line.split(" ")
            matrix_parts = parts[1:-1]
            y_part = parts[-1]
            y = [int(x) for x in y_part.strip("{}\n").split(",")]

            matrix = []
            for c in matrix_parts:
                vec = [0] * len(y)
                c = c.strip("()")
                for idx in c.split(","):
                    vec[int(idx)] = 1
                matrix.append(vec)
            input.append((matrix, y))
    return input
