from ctypes import *
from numpy import *

libml_chess = cdll.LoadLibrary("libml_chess.so");

class GameState:
    def __init__(self):
        # Create a list of integers representing the state
        integer_list = (c_int * 70)()
        libml_chess.fill_array_with_new_gamestate(integer_list)
        self.vector = ctypeslib.as_array(integer_list)

gs = GameState()
print(gs.vector)
