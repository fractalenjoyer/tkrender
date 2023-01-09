import tkinter as tk
import matplotlib.cm as cm
try:
    from tkrender import Mesh
except:
    raise ImportError(
        "tkrender not found. Please build it by running build.ps1 in the project root directory")


DISABLE_CULLING = False
FILENAME = "./objects/bulba.obj"
WIREFRAME = False


class Engine:
    def __init__(self, width=1000, height=1000, background_color="#131415", cmap="inferno", wireframe=False, disable_culling=False) -> None:
        self.width = width
        self.height = height
        self.wireframe = wireframe
        self.disable_culling = disable_culling
        self.root = tk.Tk()
        self.ctx = tk.Canvas(self.root, width=width,
                             height=height, bg=background_color)

        self.focal = [0, 0, -15]
        self.origin = [0, 0, -15]
        self.range = 20
        self.x = 0
        self.y = 0
        self.meshes = []

        self.cmap = cm.get_cmap(cmap)
        self.ctx.bind("<B1-Motion>", self.__mouse_handler)
        self.ctx.bind("<Button-1>", self.__mouse_reset)
        self.root.bind("<Key>", self.__keyboard_handler)
        self.root.bind("<MouseWheel>", self.__scroll_handler)
        self.root.title("Press H for help")

    def add_mesh(self, mesh):
        self.meshes.append(mesh)

    def redraw(self):
        self.ctx.delete("all")
        for mesh in self.meshes:
            if self.wireframe:
                self.__draw_wireframe(mesh)
            else:
                self.__draw_shaded(mesh)

    def __draw_shaded(self, mesh):
        for poly, shade in mesh.get_shaded(self.focal, self.origin, self.disable_culling):
            color = self.cmap(shade)
            polygon = [
                *map(lambda x: self.__project(x[0], x[1]), poly)]
            self.ctx.create_polygon(
                polygon, fill=f"#{int(255*color[0]):02x}{int(255*color[1]):02x}{int(255*color[2]):02x}", outline="")

    def __draw_wireframe(self, mesh):
        for poly in mesh.get_view(self.focal, self.origin, self.disable_culling):
            polygon = [
                *map(lambda x: self.__project(x[0], x[1]), poly)]
            self.ctx.create_polygon(polygon, fill="", outline="#ffffff")

    def run(self):
        self.ctx.pack()
        self.redraw()
        self.root.mainloop()

    def __mouse_handler(self, event):
        delta_x, delta_y = self.__mouse_update(event)
        for mesh in self.meshes:
            mesh.rotate_in_place(-delta_y/100, delta_x/100, 0)
        self.redraw()

    def __mouse_reset(self, event):
        self.x = event.x
        self.y = event.y

    def __keyboard_handler(self, event):
        match event.keysym:
            case "w":
                self.origin[1] += 1
            case "s":
                self.origin[1] -= 1
            case "a":
                self.origin[0] += 1
            case "d":
                self.origin[0] -= 1
            case "q":
                self.wireframe = not self.wireframe
            case "e":
                self.disable_culling = not self.disable_culling
            case "h":
                top = tk.Toplevel()
                top.title("Help")
                tk.Label(top, text="WASD to move\nQ to toggle wireframe\nE to toggle backface culling\nScroll to zoom\nLeft click to rotate", font=(
                    "Helvetica 15")).pack()
            case _:
                return
        self.redraw()

    def __scroll_handler(self, event):
        self.origin[2] += int(event.delta/120)
        self.redraw()

    def __project(self, x, y):
        return self.width*(x + self.range/2)/self.range, self.height*(y + self.range/2)/self.range

    def __mouse_update(self, event):
        delta_x = event.x - self.x
        delta_y = event.y - self.y
        self.x = event.x
        self.y = event.y
        return delta_x, delta_y


engine = Engine(wireframe=WIREFRAME, disable_culling=DISABLE_CULLING)
engine.add_mesh(Mesh(FILENAME))
engine.run()
