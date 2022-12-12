from tkmandel import Engine
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from time import sleep


cube = [
    [-1, -1, -1],
    [-1, -1, 1],
    [-1, 1, -1],
    [-1, 1, 1],
    [1, -1, -1],
    [1, -1, 1],
    [1, 1, -1],
    [1, 1, 1]
]

test = Engine(cube)
view = test.get_view()
x, y = zip(*view)
fig, ax = plt.subplots()

scat = ax.scatter(x, y)
ax.axis([-2, 2, -2, 2])
angle = 2*np.pi/360
def animate(_):
    test.rotate_y(angle)
    view = test.get_view()
    scat.set_offsets(view)
    

ani = animation.FuncAnimation(fig, animate, interval=1)
plt.show()