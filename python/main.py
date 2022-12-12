import tkmandel as rd
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation


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

focal = 1
origin = [0, 0, -2]

cube = rd.Shape(cube_points)
# cube2 = rd.Shape(cube_points)
engine = rd.Engine([cube])
x, y = zip(*engine.get_view(focal, origin))
fig, ax = plt.subplots()

scat = ax.scatter(x, y)
ax.axis([-2, 2, -2, 2])
angle = 2*np.pi/360



def animate(_):
    cube.rotate_in_place(angle, angle, 0)
    # cube2.rotate_in_place(-angle, 0, 0)
    view = engine.get_view(focal, origin)
    scat.set_offsets(view)
    

ani = animation.FuncAnimation(fig, animate, interval=1)
plt.show()