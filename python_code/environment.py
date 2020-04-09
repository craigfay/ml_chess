
import chess
import numpy

class Environment:
    def __init__(self):
        self.board = chess.Board()

    def available_actions(self):
        pass

    def state(self):
        s = numpy.zeros(64)

        for index in range(64):
            name = str(self.board.piece_at(index))
            s[index] = piece_to_int(name)

        return s
        

def piece_to_int(name):
    translation = {
        'P': 1,
        'B': 2,
        'N': 3,
        'R': 4,
        'Q': 5,
        'K': 6,
        'p': -1,
        'b': -2,
        'n': -3,
        'r': -4,
        'q': -5,
        'k': -6,
    }
    return translation.setdefault(name, 0)

e = Environment()
print(e.state())


