from day_10_parser import parse
from sys import argv

def main():
    if len(argv) >= 2:
        f = argv[1]
    else:
        f = "../data/examples/10.txt"
    input = parse(f)

    input = [
        (Matrix(A).T, vector(y)) for A, y in input
    ]

    ans = 0
    for eq in input:
        ans += solve_equation(eq[0], eq[1])

    print(ans)

def solve_equation(A, y):
    p = MixedIntegerLinearProgram(maximization=False, solver='GLPK')
    x = p.new_variable(integer=True, name="x")
    p.add_constraint(A * x == y)
    for i in x:
        p.add_constraint(i >= 0)

    p.set_objective(sum(x))
    return p.solve()


if __name__ == "__main__":
    main()
