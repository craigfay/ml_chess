
import chess
import numpy

class Environment:
    def __init__(self):
        self.__board = chess.Board()

    def available_actions(self):
        actions = []

        for move in self.__board.legal_moves:
            self.__board.push(move)

            new_board_vector = board_to_vector(self.__board)
            actions.append(new_board_vector)
            self.__board.pop()

        return actions


    def state(self):
        return board_to_vector(self.__board)
        

def board_to_vector(board):
    vector = numpy.zeros(64)

    for index in range(64):
        name = str(board.piece_at(index))
        vector[index] = piece_to_int(name)

    return vector


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

actions = e.available_actions()
for a in actions:
    print(a)
    print()


