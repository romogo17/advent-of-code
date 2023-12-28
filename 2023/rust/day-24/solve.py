from sympy import symbols, solve


def read_input(infile):
    with open(infile) as f:
        points = []
        for l in f.read().strip().splitlines():
            x = tuple([int(n) for n in l.split(" @ ")[0].split(", ")])
            v = tuple([int(n) for n in l.split(" @ ")[1].split(", ")])
            points.append((x, v))
        return points


def part2(infile):
    points = read_input(infile)

    x = symbols("x")
    y = symbols("y")
    z = symbols("z")
    vx = symbols("vx")
    vy = symbols("vy")
    vz = symbols("vz")

    X1, V1 = points[0]
    X2, V2 = points[1]
    X3, V3 = points[2]

    x1, y1, z1 = X1
    x2, y2, z2 = X2
    x3, y3, z3 = X3

    vx1, vy1, vz1 = V1
    vx2, vy2, vz2 = V2
    vx3, vy3, vz3 = V3

    sols = solve(
        [
            (x - x1) * (vy - vy1) - (y - y1) * (vx - vx1),
            (y - y1) * (vz - vz1) - (z - z1) * (vy - vy1),
            (x - x2) * (vy - vy2) - (y - y2) * (vx - vx2),
            (y - y2) * (vz - vz2) - (z - z2) * (vy - vy2),
            (x - x3) * (vy - vy3) - (y - y3) * (vx - vx3),
            (y - y3) * (vz - vz3) - (z - z3) * (vy - vy3),
        ],
        [x, y, z, vx, vy, vz],
        dict=True,
    )

    for s in sols:
        if s[vx] == int(s[vx]) and s[vy] == int(s[vy]) and s[vz] == int(s[vz]):
            print(s)
            break
    return s[x] + s[y] + s[z]


result = part2("day-24/inputs/input1.txt")
print(result)
