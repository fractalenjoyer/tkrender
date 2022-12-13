from tkinter import * 
from tkmandel import Shape
import numpy as np


def project(x, y, width, height, range):
    return width*(x + range/2)/range, height*(y + range/2)/range

def main():
    ctx.delete("all")
    for poly in object.get_poly(focal, origin):
        polygon = [*map(lambda x: project(x[0], x[1], width, height, range), poly)]
        ctx.create_polygon(polygon, fill="", outline="white")    

class Mouse:
    def __init__(self):
        self.x = 0
        self.y = 0
    
    def update(self, event):
        delta_x = event.x - self.x
        delta_y = event.y - self.y
        self.x = event.x
        self.y = event.y
        return delta_x, delta_y
    
    def reset(self, event):
        self.x = event.x
        self.y = event.y

def rotate(event):
    delta_x, delta_y = mouse.update(event)
    object.rotate_in_place(-delta_y/100, delta_x/100, 0)
    main()

focal = [0, 0, -15]
origin = [0, 0, -15]
width = height = 1000
range = 20

root = Tk()
mouse = Mouse()
object = Shape("./objects/gun.obj")
ctx = Canvas(root, width=width, height=width, bg="#131415")
    
ctx.bind("<B1-Motion>", rotate)
ctx.bind("<Button-1>", mouse.reset)

ctx.pack()
main()
root.mainloop()