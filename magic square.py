# This algorithm tries to find a magic square 3x3 for squared numbers
#
# a^2 b^2 c^2
# d^2 e^2 f^2
# g^2 h^2 i^2
#
# such as sums of each columns, rows and diagonals is the same value

import math
import random
from multiprocessing.dummy import Pool as ThreadPool

import time

# random.setstate(state.state)

print('Magic square search')


class Square:
    def __init__(self, size, maxvalue):
        self.size = size
        self.maxvalue = maxvalue
        self.values = []

    def populate(self, func, *args):
        self.values = []
        for x in range(0, self.size):
            row = []
            for y in range(0, self.size):
                row.append(func(self, args))
            self.values.append(row)

    def __contains__(self, item):
        for row in self.values:
            for value in row:
                return value == item


def power_of_2(s: Square):
    s.values = [[x ** 2 for x in row] for row in s.values]


def is_magic(s: Square) -> bool:
    # check rows, columns
    default_total = (-1, -1)
    last_total = default_total
    diagonals = [0, 0]
    for i in range(0, s.size):
        total = (0, 0, 0, 0)
        for j in range(0, s.size):
            total = (total[0] + s.values[i][j],
                     total[1] + s.values[j][i])
            if i == j:
                diagonals[0] += s.values[i][j]
            if s.size - i - 1 == j:
                diagonals[1] += s.values[i][j]
        if last_total != default_total and last_total != total:
            return False
        last_total = total

    if total[0] == total[1] and total[0] == diagonals[0] and diagonals[0] == diagonals[1]:
        print('Magic square sum ' + str(total[0]))
        return True
    else:
        return False


def gen_value_random(s: Square, *args):
    while True:
        value = random.randint(1, s.maxvalue)
        if not s.__contains__(value):
            return value


def fill_with(s: Square, value, *args):
    if isinstance(value, tuple):
        return value[0]
    return value


starting = 0


def value(s: Square, start):
    s.values = []
    used = []
    maximum = s.maxvalue
    for x in range(0, s.size):
        row = []
        for y in range(0, s.size):
            new = math.floor(start / maximum ** (y * s.size + x)) % maximum
            if new == 0 or used.__contains__(new):
                raise ValueError
            row.append(new)
            used.append(new)
        s.values.append(row)


def is_legal(s: Square):
    size = s.size
    for i in range(0, size ** 2):
        x = i % size
        y = math.floor(i / size) % size
        v = s.values[x][y]
        if v == 0:
            return False
        for j in range(i + 1, size ** 2):
            xx = j % size
            yy = math.floor(j / size) % size
            vv = s.values[xx][yy]
            if v == vv:
                return False
    return True


def gen_value_sequence(s: Square):
    global starting
    while True:
        starting += 1
        try:
            value(s, starting)
            break
        except ValueError:
            continue


tries = 0


def run_random():
    square = Square(3, 100)
    print('Random search')
    global tries
    while True:
        tries += 1
        square.populate(gen_value_random)
        power_of_2(square)
        if is_magic(square):
            print("Magic square found!")
            print(square.values)
            break


def run_sequence():
    square = Square(3, 100)
    print('Sequence search form ' + str(starting))
    global tries
    while True:
        tries += 1
        gen_value_sequence(square)
        power_of_2(square)
        if is_magic(square):
            print("Magic square found!")
            print(square.values)
            print('Starting')
            print(starting)
            break
        else:
            print('Not magic' + str(starting) + ' ' + str(square.values))


try:
    # run_random()
    # run_sequence()
    pool = ThreadPool(4)

    def func(i):
        run_random()

    pool.map(func, range(0, 4))
    pool.close()
    pool.join()
except KeyboardInterrupt:
    print('Interrupted')
    print('Count ' + str(tries))
    time = time.clock()
    print('Time spend: ' + str(round(time, 2)) + ' s')
    print('Performance ' + str(round(tries / time, 0)) + ' ties/s')
