from ctypes import *
from numpy import *

libml_chess = cdll.LoadLibrary("libml_chess.so");

integer_list = (c_int * 70)()
libml_chess.fill_array_with_gamestate(integer_list)

gamestate = ctypeslib.as_array(integer_list)
print(gamestate)

