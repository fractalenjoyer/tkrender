import tkmandel as rd
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from time import time


def make_donut(density1, density2):
    start = rd.Shape([[0.6, 0, 0]])
    new_points = []
    for _ in range(density2):
        new_points += start.get_points()
        start.rotate_in_place(0, 0, 2*np.pi/density2)

    for arr in new_points:
        arr[0] += 1

    ring = rd.Shape(new_points)

    for _ in range(density1):
        ring.rotate_in_place(0, 2*np.pi/density1, 0)
        new_points += ring.get_points()

    return rd.Shape(new_points)

donut = make_donut(40, 20)

engine = rd.Engine([donut])


focal = 1
origin = [0, 0, -2]
engine.get_view(focal, origin)
x, y = zip(*engine.get_view(focal, origin))
fig, ax = plt.subplots()

colors = np.linspace(0, 100, len(x))
scat = ax.scatter(x, y)#, c=colors, cmap="hsv")
ax.axis([-2, 2, -2, 2])
angle = 2*np.pi/360

frame = 0
start = time()

def animate(_):
    global frame, start
    donut.rotate_in_place(angle, angle, angle)
    view = engine.get_view(focal, origin)
    scat.set_offsets(view)
    frame += 1
    if (delta := time() - start) >= 1:
        start = time()
        ax.set_title(f"{frame/delta:.2f} fps")
        frame = 0

ani = animation.FuncAnimation(fig, animate, interval=1)
plt.show()
