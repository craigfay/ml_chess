from ctypes import *
from numpy import *

libml_chess = cdll.LoadLibrary("libml_chess.so");

class GameState:
    def __init__(self):
        # Creating a list of integers representing the state
        integer_list = (c_int * 70)()
        libml_chess.fill_array_with_new_gamestate(integer_list)
        self.vector = ctypeslib.as_array(integer_list)

    def is_checkmate(self):
        # Converting self.vector to an array of C ints
        integer_list = (c_int * 70)() 
        for i in range(70):
            integer_list[i] = self.vector[i]

        answer = libml_chess.numeric_gamestate_is_checkmate(integer_list)
        return bool(answer)



gs = GameState()
print(gs.vector)
print(gs.is_checkmate())

