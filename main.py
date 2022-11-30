import mandelgen as mg
from PIL import Image
import numpy as np

dim = 1000

def main():
    # img = np.zeros((dim,dim, 3), dtype=np.uint8)
    # for x in range(dim):
    #     for y in range(dim):
    #         a, b = 3*x / dim - 2, 3* y / dim - 1.5
    #         img[x, y, 0] = mg.calc(a,b)
    img = mg.get_mandel(dim)
    Image.fromarray(img).save("mandelbrot.png")
main()