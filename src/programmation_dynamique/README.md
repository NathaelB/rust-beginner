<h3 align="center">Dynamic Programming</h3>
<p align="center">Part on dynamic programming ⚡</p>

![Lol](https://i.imgur.com/vAXYhJr.jpeg)

## 📚 Exercise 1 (courses)
The aim of this exercise is to implement the algorithm of choice of valued courses seen in the course, this one being based on
dynamic programming.
A course is encoded by an array with three entries: its start date, its end date, and its value. A set of courses
is an array of courses.

### Question 1
Complete the `random_courses` function which generates a set of random courses.

### Question 2
Complete the function `sort_courses` which sorts a set of courses by increasing end date.
And create the function `calcul_pred` whose purpose is to fill the array pred in order to check
the following condition. For each round `i`, the course `Pred[i]` is the latest possible ending course, compatible
with course `i` and ending strictly with the beginning of course `i`. If no such course exists, `Pred[i] = -1` is kept.

### Question 3

## 📚 Exercise 2 (Seccotine)
Seccotine, the famous adventurer, arrives in the last room of the castle she is exploring.
This room is paved and on each slab is placed a small pile of gold coins.
This room is modelled by a matrix M of dimension n x n where each cell represents a tile
and whose values correspond to the number of gold coins placed on the different tiles.
Seccotine enters through the (0,0) square, top left, and must exit through the
(n-1, n-1) at the bottom right. Unfortunately, an ugly monster is chasing Seccotine and it is impossible
cannot move upwards or to the left, otherwise she will be eaten.
otherwise she will be eaten. Seccotine obviously wants to maximise the treasure she has collected during her
Seccotine is obviously trying to maximise the treasure she has collected during her time in the castle, so she wants to get across the room and collect as much gold as possible.

To model the movement of Seccotine, we call a sequence of cells of the matrix M
consecutively adjacent, i.e. with a common edge. A path is said to be admissible
if it starts from the position (0,0) and goes only to the right or downwards, i.e.
that it uses only steps of the type (i,j) -> (i,j+1) or (i,j) -> (i+1,j).
The value of a path is the sum of the values of the cells it contains.

The aim of the exercise is to write a dynamic programming algorithm
to calculate the largest admissible path of the matrix M.

### Question 1
Saccotine thinks of applying a gluttonous algorithm to find the admissible path of
value: i.e. if she is in cell (i,j), she will choose among cells (i,j+1) and
box (i,j+1) and (i+1,j) the one with the highest value.

### Question 2
Derive a dynamic programming type algorithm to calculate
the largest value of a permissible path ending at the (n-1, n-1) cell of a matrix M.

### Question 3
Write an algorithm to display the cells traversed by an admissible path
of maximum value of M.