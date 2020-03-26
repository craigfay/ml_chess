from ctypes import *
from numpy import *
from random import Random

libml_chess = cdll.LoadLibrary("libml_chess.so");


class IntArray2D(Structure):
    _fields_ = [
        ("array", (c_int * 2) * 3),
    ]


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

    def is_stalemate(self):
        # Converting self.vector to an array of C ints
        integer_list = (c_int * 70)() 
        for i in range(70):
            integer_list[i] = self.vector[i]

        answer = libml_chess.numeric_gamestate_is_stalemate(integer_list)
        return bool(answer)

    def white_vs_black_material(self):
        # Converting self.vector to an array of C ints
        integer_list = (c_int * 70)() 
        for i in range(70):
            integer_list[i] = self.vector[i]

        answer = (c_int * 2)() 
        libml_chess.numeric_gamestate_material_values(integer_list, answer)
        return ctypeslib.as_array(answer)

    def legal_next_states(self):
        # Converting self.vector to an array of C ints
        integer_list = (c_int * 70)() 
        for i in range(70):
            integer_list[i] = self.vector[i]

        intArray2D = IntArray2D()
        libml_chess.example(intArray2D.array)
        a = ctypeslib.as_array(intArray2D.array)
        return a



def randomInt(min, max):
    return Random().randrange(min, max)

    
gs = GameState()
print(gs.legal_next_states())

