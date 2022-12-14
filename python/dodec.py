import tktp as rd #type: ignore
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation

phi = (1 + np.sqrt(5))/2

dodecahedron_vectors = [    
    [1, 1, 1],
    [1, 1, -1],
    [1, -1, 1],
    [1, -1, -1],
    [-1, 1, 1],
    [-1, 1, -1],
    [-1, -1, 1],
    [-1, -1, -1],
    [0, 1/phi, phi],
    [0, 1/phi, -phi],
    [0, -1/phi, phi],
    [0, -1/phi, -phi],
    [1/phi, phi, 0],
    [1/phi, -phi, 0],
    [-1/phi, phi, 0],
    [-1/phi, -phi, 0],
    [phi, 0, 1/phi],
    [phi, 0, -1/phi],
    [-phi, 0, 1/phi],
    [-phi, 0, -1/phi]
]

focal = 1
origin = [0, 0, -2]

dice = rd.Shape(dodecahedron_vectors)
# cube2 = rd.Shape(cube_points)
engine = rd.Engine([dice])
x, y = zip(*engine.get_view(focal, origin))
fig, ax = plt.subplots()

scat = ax.scatter(x, y)
ax.axis([-2, 2, -2, 2])
angle = 2*np.pi/360



def animate(_):
    dice.rotate_in_place(angle, angle, 0)
    # cube2.rotate_in_place(-angle, 0, 0)
    view = engine.get_view(focal, origin)
    scat.set_offsets(view)
    

ani = animation.FuncAnimation(fig, animate, interval=1)
plt.show()