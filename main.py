import mandelgen as mg
from tkinter import *
from PIL import Image
import numpy as np
from timeit import timeit

dim = 1000

def main():
    print(timeit(lambda: mg.get_mandel(dim, 255), number=10))
    #Image.fromarray(img).show()
main()