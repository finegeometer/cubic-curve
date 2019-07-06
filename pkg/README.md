Draw the unique cubic curve through nine points.

# Usage

Click a red dot to pick it up.
Move the mouse to move it around.
Click again to put it down.

# Implementation

Coded in Rust, compiled to WASM.

## Calculating the curve

Suppose we want to calculate the cubic curve through nine points: `(x_1, y_1)` through `(x_9, y_9)`.
I claim that this is the equation of the cubic:
```
    [ x_1*x_1*x_1, x_1*x_1*y_1, x_1*y_1*y_1, y_1*y_1*y_1, x_1*x_1, x_1*y_1, y_1*y_1, x_1, y_1, 1 ]
    [ x_2*x_2*x_2, x_2*x_2*y_2, x_2*y_2*y_2, y_2*y_2*y_2, x_2*x_2, x_2*y_2, y_2*y_2, x_2, y_2, 1 ]
    [ x_3*x_3*x_3, x_3*x_3*y_3, x_3*y_3*y_3, y_3*y_3*y_3, x_3*x_3, x_3*y_3, y_3*y_3, x_3, y_3, 1 ]
    [ x_4*x_4*x_4, x_4*x_4*y_4, x_4*y_4*y_4, y_4*y_4*y_4, x_4*x_4, x_4*y_4, y_4*y_4, x_4, y_4, 1 ]
det [ x_5*x_5*x_5, x_5*x_5*y_5, x_5*y_5*y_5, y_5*y_5*y_5, x_5*x_5, x_5*y_5, y_5*y_5, x_5, y_5, 1 ] = 0
    [ x_6*x_6*x_6, x_6*x_6*y_6, x_6*y_6*y_6, y_6*y_6*y_6, x_6*x_6, x_6*y_6, y_6*y_6, x_6, y_6, 1 ]
    [ x_7*x_7*x_7, x_7*x_7*y_7, x_7*y_7*y_7, y_7*y_7*y_7, x_7*x_7, x_7*y_7, y_7*y_7, x_7, y_7, 1 ]
    [ x_8*x_8*x_8, x_8*x_8*y_8, x_8*y_8*y_8, y_8*y_8*y_8, x_8*x_8, x_8*y_8, y_8*y_8, x_8, y_8, 1 ]
    [ x_9*x_9*x_9, x_9*x_9*y_9, x_9*y_9*y_9, y_9*y_9*y_9, x_9*x_9, x_9*y_9, y_9*y_9, x_9, y_9, 1 ]
    [  x * x * x ,  x * x * y ,  x * y * y ,  y * y * y ,  x * x ,  x * y ,  y * y ,  x ,  y , 1 ]
```

### Proof

#### This is a cubic.

The determinant is linear in each of its rows. Therefore, the left side of this equation is a linear combination of elements of the bottom row, with coefficients calculated from the other rows. Since the other rows are filled with constants, this is a linear combination of cubics with constant coefficients, which is always a cubic.

#### This goes through the nine points.

If `(x, y)` = `(x_i, y_i)`, then two rows of the matrix are identical. Then the determinant is zero, and the equation is satisfied.

So `(x_i, y_i)` is on the curve.


## Drawing the curve

Now we have a curve: `f(x,y) = 0`, where `f(x,y) = a*x*x*x + b*x*x*y + c*x*y*y + d*y*y*y + e*x*x + f*x*y + g*y*y + h*x + i*y + j`. How do we draw it?

I chose to do the rendering in OpenGL, using a fragment shader. This means the GPU will run some code for each pixel to decide what color to paint it.

So the question is: How should a pixel decide what color to paint itself?

### Idea 0: Just use the equation!

**Proposed algorithm: If `f(x,y) = 0`, color the pixel black. Otherwise, color it white.**

Unfortunately, this doesn't work. The equation is extremely unlikely to hold exactly, so we just get a blank screen.

### Idea 1: The obvious fix.

We want to color the points that *almost* fit the equation, not just those that fit it exactly.

**Proposed algorithm: Pick a small positive number ε. If `-ε < f(x,y) < ε`, color the pixel black. Otherwise, color it white.**

Unfortunately, this also doesn't work. (Todo: create picture of why.)

### Idea 2: A better solution.

We want the line to be of constant width. In other words, we want to color those pixels that are within ε of being exactly on the curve.

Since ε is small, we can make things easier by using a linear approximation to f(x).

**Proposed algorithm: Pick a small positive number ε. If `-ε < f(x,y) / |∇f(x,y)| < ε`, color the pixel black. Otherwise, color it white.**

This works really well, and is roughly the algorithm used here.

A few quirks with this algorithm:

* If the curve is doubled (like `f(x,y) * f(x,y) = 0`), it is drawn with twice the normal thickness.

  * Reason: `(f*f)(x,y) / |∇(f*f)(x,y)| = f(x,y) * f(x,y) / (2 * f(x,y) * |∇f(x,y)|) = (1/2) f(x,y) / |∇f(x,y)|`

* A sufficiently fast-growing function (like `f(x,y) = exp(x / ε)`) may cause the entire plane to be colored black.


# License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
