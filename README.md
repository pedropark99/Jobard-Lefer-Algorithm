# Implementing the Jobard and Lefer algorithm 

This small project implements the Jobard and Lefer (1997) algorithm
in three different languages (C, C++ and Rust). This algorithm is used for drawing non-overlapping
and evenly-spaced curves in a flow field (also called of a vector field).

The algorithm is described in a scientific paper:

Jobard, Bruno, and Wilfrid Lefer. 1997. “Creating Evenly-Spaced Streamlines of Arbitrary Density.” In Visualization in Scientific Computing ’97, edited by Wilfrid Lefer and Michel Grave, 43–55. Vienna: Springer Vienna.

But you mmight find my article about this algorithm an useful resource as well:

<https://pedro-faria.netlify.app/posts/2024/2024-02-19-flow-even/en/>


# An example of output

Just to clarify the objective of the algorithm,
the two images below demonstrates the effect of the algorithm
in the process of drawing multiple curves in a flow field.

First, an example of curves drawn without the Jobard and Lefer (1997) algorithm:

![Curves drawn without the Jobard and Lefer algorithm](https://pedro-faria.netlify.app/posts/2024/2024-02-19-flow-even/overlap.png)


Now, the same example, but this time, using the Jobard and Lefer (1997) algorithm. You
can see that we get non-overlapping curves that are evenly-spaced between each other:

![Non-Overlapping curves drawn with the Jobard and Lefer algorithm](https://pedro-faria.netlify.app/posts/2024/2024-02-19-flow-even/even_curves2.png)
