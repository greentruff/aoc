import fileinput
import functools
import os

def debug(str: str):
    if os.environ.get('DEBUG', None):
        print(str)

class Game:
    id: int
    max_count: dict[str, int]
    def __init__(self, line: str):
        # Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        debug(line)
        game_seperator_pos = line.find(': ')
        self.id = int(line[4:game_seperator_pos])
        debug(f'-- ID: {self.id}')

        self.max_count = {}
        games = line[game_seperator_pos+2:].split('; ')
        for game in games:
            balls = game.split(', ')
            for ball in balls:
                debug(f"-- B: '{ball}'")
                count, color = ball.split(' ', 1)
                if int(count) > self.max_count.get(color, 0):
                    self.max_count[color] = int(count)
    
    def is_possible(self, bag_content) -> bool:
        for color, bag_count in bag_content.items():
            if self.max_count.get(color, 0) > bag_count:
                debug(f"FAILED {color}:{bag_count} < {self.max_count.get(color, 0)}")
                return False
        return True
    
    def power(self) -> int:
        return functools.reduce(lambda x, y: x*y, self.max_count.values())

id_sum = 0
power_sum = 0
for line in fileinput.input():
    g = Game(line.strip())
    if g.is_possible({'red': 12, 'green': 13, 'blue': 14}):
        id_sum = id_sum + g.id
    power_sum = power_sum + g.power()

print(f"Part 1: {id_sum}")
print(f"Part 2: {power_sum}")