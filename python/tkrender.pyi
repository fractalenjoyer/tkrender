class Mesh:
    """
    A class representing a 3D mesh object.
    
    :param path: The path to the .obj file to load.
    """
    def __init__(self, path: str) -> None: ...
    """
    Rotate the mesh in place.
    
    :param x: The angle to rotate around the x axis in radians.
    :param y: The angle to rotate around the y axis in radians.
    :param z: The angle to rotate around the z axis in radians.
    """
    def rotate_in_place(self, x: float, y: float, z: float) -> None: ...
    """
    Rotates the mesh and returns all of its points.
    
    :param x: The angle to rotate around the x axis in radians.
    :param y: The angle to rotate around the y axis in radians.
    :param z: The angle to rotate around the z axis in radians.
    :return: A list of all of the points in the mesh.
    """
    def rotate(self, x: float, y: float, z: float) -> list[list[float]]: ...
    """
    Gets a list of all of the polygons in the mesh.
    Backface culling is enabled by default but can be disabled by setting disable_culling to True.
    
    :param focal: The focal point of the camera.
    :param origin: The origin of the camera.
    :param disable_culling: Whether or not to disable backface culling.
    :return: A list of all of the polygons in the mesh.
    """
    def get_view(self, focal: list[float], origin: list[float], disable_culling: bool = False) -> list[list[list[float]]]: ...
    """
    Gets a list of tuples containing the polygons in the mesh and their respective shading.
    Just like with get_view backface culling is enabled by default but can be disabled by setting disable_culling to True.
    
    :param focal: The focal point of the camera.
    :param origin: The origin of the camera.
    :param disable_culling: Whether or not to disable backface culling.
    :return: A list of tuples containing the polygons in the mesh and their respective shading.
    """
    def get_shaded(self, focal: list[float], origin: list[float], disable_culling: bool = False) -> list[tuple[list[list[float]], float]]: ...
    