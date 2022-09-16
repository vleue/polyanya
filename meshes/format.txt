DIFFERENCES BETWEEN VERSION 1 AND 2:
- polygon.p: For each polygon p[i], it shares v[i-1] and v[i] instead of v[i]
  and v[i+1].
  (An easy way to convert is to simply move the last element of each polygon's
  p array to the start of the array.)

Mesh map file format version 2 is as defined:

The first line is "mesh", the header.
The second line is the version of the format, 2.
The third line contains two integers: V and P.
    V is the number of vertices in the mesh.
    P is the number of polygons in the mesh.
Then follows the vertex section of the file, containing V lines.
Then follows the polygon section of the file, containing P lines.
End of file.

This is only the recommended format. As the length of each line (in numbers)
will be known, this format is whitespace insensitive. This means that all
recommended whitespace (spaces, new lines) can be replaced with any other space.

Each section defines an array of that object, with each line being a single
object. We will define S[i] to be the ith object in section S. For example,
polygon[1] is the second polygon of the mesh.

Assume that the polygons do not contain any self-intersections and are convex.


A vertex is defined by:
    x, y: number.
        The x and y coordinates of the point.
    neighbours: integer.
        How many vertices the vertex is connected to.
        This is equivalent to the number of edges and polygons the vertex is
        neighbouring.
    p: an array of indices of size neighbours.
        The neighbouring polygons.
        Each integer represents an index of the polygon array.
        Ordered in counterclockwise order with an arbirary start.
        If the polygon is not defined (it is outside the map or an obstacle),
        a value of -1 is used.
        Any two "adjacent" obstacles should be merged into one.

        Note that Polyanya usually does not use this at all - all it uses it for
        will be identifying how many obstacles are around a point, and the
        array itself is only used when that number is 0. However, even when it
        is used in this fashion the ordering does not matter.

A polygon is defined by:
    n: integer.
        How many vertices the polygon has.
    v: array of indices of size n.
        The vertices of the polygon.
        Each integer represents an index of the vertex array.
        Should be sorted such that iterating through v goes through the
        vertices in counterclockwise order.
        The start of the array is arbitrary.
    p: array of indices of size n.
        The neighbouring polygons.
        Each integer represents an index of the polygon array.
        Ordered such that for each polygon p[i], p[i] and this polygon share
        the edge of v[i] and v[i-1], wrapping around for the first polygon.
        If the polygon is not defined (it is outside the map or an obstacle),
        a value of -1 is used.

When serialising an object into a line of the format, store all numbers in order
and separate them by spaces. As the length of all arrays have been defined
before the array, arrays are stored as consecutive numbers.

An example of this format is as shown:

BEGIN FILE
mesh
2
4 2
0.0 0.0 2 0 -1
1.5 0.0 3 0 1 -1
1.5 1.5 2 1 -1
0.0 1.5 3 -1 1 0
3 0 1 3 -1 -1 1
3 1 2 3 0 -1 -1
END OF FILE

This represents this mesh:

0         1
 X-------X
 |      /|
 | 0   / |
 |    /  |
 |   /   |
 |  /    |
 | /   1 |
 |/      |
 X-------X
3         2

X denotes a point of the polygon.
