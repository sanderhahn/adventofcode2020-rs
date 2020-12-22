game = """Player 1:
42
29
12
40
47
26
11
39
41
13
8
50
44
33
5
27
10
25
17
1
28
22
6
32
35

Player 2:
19
34
38
21
43
14
23
46
16
3
36
31
37
45
30
15
49
48
24
9
2
18
4
7
20
"""

example_game = """Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
"""

stacks = []
for (player, player) in enumerate(game.split("\n\n")):
  stack = []
  for (index, line) in enumerate(player.splitlines()):
    if index > 0:
      stack.append(int(line))
  stacks.append(stack)

def recursive_game(lefts, rights, gamestates, game):
  while len(lefts) > 0 and len(rights) > 0:
    left = lefts.pop(0)
    right = rights.pop(0)
    gamestate = [lefts.copy(), rights.copy()]
    if gamestate in gamestates:
      return True
    if len(lefts) >= left and len(rights) >= right:
      winner = recursive_game(lefts[:left], rights[:right], [], game+1)
    else:
      winner = left > right

    if winner:
      lefts.append(left)
      lefts.append(right)
    else:
      rights.append(right)
      rights.append(left)

    gamestates.append(gamestate)

  return len(lefts) > len(rights)

winner = recursive_game(stacks[0], stacks[1], [], 1)
if winner:
  stack = stacks[0]

level = 1
score = 0
while len(stack) > 0:
  bottom = stack.pop()
  score += bottom * level
  level += 1

print(score)
