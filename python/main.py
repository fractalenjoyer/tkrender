import tkinter as tk 
from tkrender import Shape #type: ignore
import numpy as np
from timeit import timeit
import matplotlib.cm as cm

def project(x, y, width, height, range):
    return width*(x + range/2)/range, height*(y + range/2)/range



def main():
    ctx.delete("all")
    cmap = cm.get_cmap("inferno")
    for poly, normal in object.get_culled(focal, origin):
        color = cmap(abs(np.dot(normal, [0, 0, 1])))
        polygon = [*map(lambda x: project(x[0], x[1], width, height, range), poly)]
        ctx.create_polygon(polygon, fill=format(f"#{int(255*color[0]):02x}{int(255*color[1]):02x}{int(255*color[2]):02x}"), outline="")    

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

def move(event):
    global origin
    match event.keysym:
        case "w":
            origin[1] += 1
        case "s":
            origin[1] -= 1
        case "a":
            origin[0] += 1
        case "d":
            origin[0] -= 1
        case _: 
            return
    main()

def zoom(event):
    global origin
    origin[2] += int(event.delta/120)
    main()

focal = [0, 0, -15]
origin = [0, 0, -15]
range = 20
width = height = 1000

root = tk.Tk()
mouse = Mouse()
object = Shape("./objects/teapot.obj")
ctx = tk.Canvas(root, width=width, height=width, bg="#131415")
    
ctx.bind("<B1-Motion>", rotate)
ctx.bind("<Button-1>", mouse.reset)
root.bind("<Key>", move)
root.bind("<MouseWheel>", zoom)

ctx.pack()
main()
root.mainloop()