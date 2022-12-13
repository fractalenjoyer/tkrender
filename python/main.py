from tkinter import * 
from tkmandel import Shape, Engine
import numpy as np

cube_points = [
    [-1, -1, -1],
    [-1, -1, 1],
    [-1, 1, -1],
    [-1, 1, 1],
    [1, -1, -1],
    [1, -1, 1],
    [1, 1, -1],
    [1, 1, 1]
]

cube = Shape(cube_points)
engine = Engine([cube])
focal = 1
origin = [0, 0, -2]

root = Tk()

ctx = Canvas(root, width=400, height=400)

view = map(lambda x: 400*(x + 2)/4, np.array(engine.get_view(focal, origin)).flatten())
ctx.create_polygon(*view)

ctx.pack()
root.mainloop()