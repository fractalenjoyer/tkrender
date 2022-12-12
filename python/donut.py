import tkmandel as rd
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation


def make_donut(density):
    start = rd.Shape([[0.5, 0, 0]])
    new_points = []
    for _ in range(20):
        new_points += start.get_points()
        start.rotate_in_place(0, 0, 2*np.pi/20)

    for arr in new_points:
        arr[0] += 1

    ring = rd.Shape(new_points)

    for _ in range(density):
        ring.rotate_in_place(0, 2*np.pi/density, 0)
        new_points += ring.get_points()

    return rd.Shape(new_points)

donut = make_donut(50)

engine = rd.Engine([donut])


focal = 1
origin = [0, 0, -2]
engine.get_view(focal, origin)
x, y = zip(*engine.get_view(focal, origin))
fig, ax = plt.subplots()

scat = ax.scatter(x, y)
ax.axis([-2, 2, -2, 2])
angle = 2*np.pi/360


def animate(_):
    donut.rotate_in_place(angle, angle, angle)
    view = engine.get_view(focal, origin)
    scat.set_offsets(view)


ani = animation.FuncAnimation(fig, animate, interval=1)
plt.show()
