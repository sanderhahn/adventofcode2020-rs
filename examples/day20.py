from math import sqrt, floor
from pprint import pprint
from math import sqrt

tiles = {}
side_to_tiles = {}


def tile_line(lines, index=0):
    return "".join([line[index] for line in lines])


def store_side(name, side):
    if not side in side_to_tiles:
        side_to_tiles[side] = []
    side_to_tiles[side].append(name)
    side = side[::-1]
    if not side in side_to_tiles:
        side_to_tiles[side] = []
    side_to_tiles[side].append(name)


def get_sides(name):
    lines = tiles[name]
    return [
        lines[0],
        tile_line(lines, -1),
        lines[-1],
        tile_line(lines, 0),
    ]


def store_sides(name):
    for side in get_sides(name):
        store_side(name, side)


def rotate_tile(name):
    "rotate clockwise"
    lines = tiles[name]
    tiles[name] = [tile_line(lines, i)[::-1] for (i, _) in enumerate(lines)]


def flip_tile(name):
    "flip over the diagonal axis"
    lines = tiles[name]
    tiles[name] = [tile_line(lines, i) for (i, _) in enumerate(lines)]


for tile in open("inputs/day20.txt").read().strip().split("\n\n"):
    lines = tile.split("\n")
    name = lines.pop(0)
    name = name.lstrip("Tile ").rstrip(":")
    tiles[name] = lines
    store_sides(name)

neighbors = {}
for (side, names) in side_to_tiles.items():
    if len(names) > 1:
        left = names.pop()
        right = names.pop()
        if not left in neighbors:
            neighbors[left] = set()
        neighbors[left].add(right)
        if not right in neighbors:
            neighbors[right] = set()
        neighbors[right].add(left)

width = int(sqrt(float(len(tiles))))
corners = set([tile for tile in tiles if len(neighbors[tile]) == 2])
sides = set([tile for tile in tiles if len(neighbors[tile]) == 3])
center = set([tile for tile in tiles if len(neighbors[tile]) == 4])

assert(len(corners) == 4)
assert(len(sides) == (width - 2) * 4)
assert(len(center) == (width - 2) ** 2)

part1 = 1
for tile in corners:
    part1 *= int(tile)
print(part1)

grid = [[0] * width for i in range(width)]
done = set()


def print_grid(grid):
    for line in grid:
        print(line)
    print()


def print_tile(tile):
    for line in tiles[tile]:
        print(line)
    print()


def place(x, y, tile):
    grid[y][x] = tile
    done.add(tile)


def number_of_neighbors(x, y, width):
    in_center = x > 0 and y > 0 and x < width - 1 and y < width - 1
    if in_center:
        return 4
    in_corner = (x == 0 and y == 0) or (x == width - 1 and y == 0) or (x ==
                                                                       0 and y == width - 1) or (x == width - 1 and y == width - 1)
    if in_corner:
        return 2
    return 3


def pick_neighbors(x, y, width):
    left_neighbors = set()
    if x > 0:
        left_neighbors = neighbors[grid[y][x-1]]
    top_neighbors = set()
    if y > 0:
        top_neighbors = neighbors[grid[y-1][x]]

    if left_neighbors and top_neighbors:
        pick_neighbors = (left_neighbors & top_neighbors) - done
    elif left_neighbors:
        pick_neighbors = left_neighbors
    else:
        pick_neighbors = top_neighbors

    pick_neighbors = [pick for pick in pick_neighbors if not pick in done and len(
        neighbors[pick]) == number_of_neighbors(x, y, width)]
    if len(pick_neighbors) != 1:
        raise Exception('internal error')
    return next(iter(pick_neighbors))


def cantor(x, y):
    "http://szudzik.com/ElegantPairing.pdf"
    z = x ** 2
    z += 3 * x
    z += 2 * x * y
    z += y + y ** 2
    z /= 2
    return int(z)


def cantor_unpair(z):
    i = floor((-1 + sqrt(1 + 8 * z)) / 2)
    x = z - int((i * (1 + i)) / 2)
    y = int((i * (3 + i)) / 2) - z
    return [x, y]


start = next(iter(corners))
place(0, 0, start)
init = iter(neighbors[start])
place(1, 0, next(init))
place(0, 1, next(init))

for i in range(3, cantor(width, width)):
    x, y = cantor_unpair(i)
    if x >= width or y >= width:
        continue
    n = pick_neighbors(x, y, width)
    place(x, y, n)

# flip grid over diagonal axis


def flip_grid():
    global grid
    return [[grid[x][y] for (x, _) in enumerate(grid)] for (y, _) in enumerate(grid)]


print_grid(grid)

top = 0
right = 1
bottom = 2
left = 3


def match_vertically(one, two):
    one = get_sides(one)
    two = get_sides(two)
    return one[bottom] == two[top]


def match_horizontally(one, two):
    print("one", one)
    print("two", two)
    one = get_sides(one)
    two = get_sides(two)
    return one[right] == two[left]


def solve_bottom(top, bottom):
    for _ in range(2):
        for _ in range(4):
            if match_vertically(top, bottom):
                return True
            print("rotate bottom")
            rotate_tile(bottom)
        print("flip bottom")
        flip_tile(bottom)
    return False


def solve_right(left, right):
    for _ in range(2):
        for _ in range(4):
            if match_horizontally(left, right):
                return True
            print("rotate right")
            rotate_tile(right)
        print("flip right")
        flip_tile(right)
    return False


def solve_grid():
    for i in range(1, cantor(width, width)):
        x, y = cantor_unpair(i)
        if x >= width or y >= width:
            continue
        print(x, y)
        print(grid[y][x])
        if y > 0 and not solve_bottom(grid[y-1][x], grid[y][x]):
            return False
        if x > 0 and not solve_right(grid[y][x-1], grid[y][x]):
            return False
    return True


def solve():
    global start
    for _ in range(2):
        bottom = grid[1][0]
        right = grid[0][1]
        for _ in range(2):
            for _ in range(4):
                if solve_grid():
                    return True
                print("rotate start")
                rotate_tile(start)
            print("flip start")
            flip_tile(start)
        print("flip grid")
        flip_grid()
    return False


if not solve():
    raise Exception('failed to solve')


def tile_line(lines, index=0):
    return "".join([line[index] for line in lines])


def stitch_row(row):
    row = [tiles[tile] for tile in row]
    combined = []
    for y in range(len(row[0])):
        combined.append("".join([tile[y] for tile in row]))
    return combined


def stitch_grid():
    stitched = [stitch_row(row) for row in grid]
    stitched = [item for sublist in stitched for item in sublist]
    return stitched


def strip_tiles():
    for (name, tile) in tiles.items():
        tiles[name] = [line[1:-1] for line in tile][1:-1]


strip_tiles()

for line in stitch_grid():
    print(line)
