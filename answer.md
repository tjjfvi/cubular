
# Rust, XYZ bytes

*Because it's "enough of a challenge as it is" I didn't "make it
[tag:code-golf] just to spice it up a bit more".*

The source code is up on github at [tjjfvi/cubular](https://github.com/tjjfvi/cubular).

If you'd like to play with the cube and see how it's solved, I've created [an
interactive visualization](https://tjjfvi.github.io/cubular).

If you'd like to read about how it's solved, I've written a long detailed
explanation with interactive illustrations of the process.

## Explanation

### Coordinate System

This explanation will use a slightly different coordinate system than the challenge.

`X` is left-to-right, `Y` is top-to-bottom, and `Z` is front-to-back.

Here are 3 different visualizations of the axes:
```
   z
  / E F
 /  G H
* - - x
| A B
| C D
y

A B  E F
C D  G H
```
```
0 x  z ·
y ·  · ·
```
```
Positions as 'xyz':
000 100   001 101
010 110   011 111
```

Rotations are clockwise looking down the axis (right-hand rule).

Here is a visualization of the 3 unit rotations:
```
cube:       X1(cube):
0 x  z ·    z ·  · ·
y ·  · ·    0 x  y ·

cube:       Y1(cube):
0 x  z ·    x ·  0 z
y ·  · ·    · ·  y ·

cube:       Z1(cube):
0 x  z ·    y 0  · z
y ·  · ·    · x  · ·
```

### 0-indexing

The original challenge 1-indexes the pieces, which complicates
the modulo arithmetic. This explanation will use 0-indexing, where the pieces
are numbered 0-8.

```
1-indexing
1 2 3 4 5 6 7 8 9
2 3 4 5 6 7 8 9 1
...

0-indexing
0 1 2 3 4 5 6 7 8
1 2 3 4 5 6 7 8 0
...
```

### Parity

Pieces are locked in to a checkerboard pattern; pieces from odd places
can only go to odd places, and likewise for even.

This means that there are actually 18 kinds of pieces.

```
0 in an even position -->  0 1 2 3 4 5 6 7 8
                           1 2 3 4 5 6 7 8 0 <-- 0 in an odd position
                           ...
```

This brings us to...

### Alternate Notation

Because there are 18 kinds of pieces, it's helpful to have 18 distinct symbols.
From now on, this explanation will use an adapted, alpha-numeric notation, where
all even-parity pieces are numbers, and all odd-parity pieces are letters.

```
0-indexed
0 1 2 3 4 5 6 7 8
1 2 3 4 5 6 7 8 0
2 3 4 5 6 7 8 0 1
3 4 5 6 7 8 0 1 2
...

Alpha-numeric
0 a 1 b 2 c 3 d 4
a 1 b 2 c 3 d 4 e
1 b 2 c 3 d 4 e 5
b 2 c 3 d 4 e 5 f
...
```

### Links

Throughout this explanation, links to cube states will be used to illustrate the
process. This is [the solved state of the cube][solved], and this is an [example
of a small algorithm][stage1-step1-A].

### Approach to Solving

To solve the cube, we iterate through the positions in some order and solve each of
them without unsolving the previous positions.

Positions can be in two states: solved & unsolved. When solving a position,
pieces in solved positions may be moved but must be restored to ensure the solved
positions are ultimately unchanged.

A position is solved by the following process:
1. Locate a piece of the correct value in any unsolved position.
2. Move the piece to the correct position without ultimately changing any solved
   position.

\#1 is fairly self-explanatory, so the rest of this explanation focuses on #2.

At various times throughout solving the cube, we'll denote an unsolved section of the
cube as the "pool". We'll then divide #2 into 3 parts:

1. Move the piece into the pool.
2. Move the piece around in the pool.
3. Move the piece out of the pool into the correct position.

To accomplish this, we'll need to define two helpers:
1. We'll figure out how to move a piece from any position in the pool to any
   other position of the same parity in the pool.
2. Then, for each unsolved position outside of the pool, we'll choose a convenient
   "source" position in the pool and figure out a list of moves that will move a piece
   from the source position to the position we're solving.

Using these two helpers, we can refine the above process:
1. Move the piece into the pool.
   - Execute the reverse of the moves supplied by helper #2.
2. Move the piece around in the pool.
   - Use helper #1.
3. Move the piece out of the pool into the correct position.
   - Execute the moves supplied by helper #2.

We will now formalize and define this logic.

### Overview

At a high level, the solving method is broken into three stages.
1. Solving the outer shell (everything but the center 5×5×5)
2. Solving the inner shell (the center 5×5×5 excluding the center 3×3×3)
3. Solving the center 3×3×3

Each stage consists of a few steps.

### Step Structure

In each step, the puzzle positions are paritioned into 3 classifications:
1. 'active': unsolved positions that will be solved by the step
2. 'done': previously solved positions
3. 'pool': unsolved positions

A step is executed by iterating through the 'active' positions and solving each
position by executing the following prodecure:
1. Locate a piece of the correct value in any unsolved position.
2. Move the piece into the pool.
3. Move the piece within the pool to a 'source' position in the pool.
4. Move the piece from the 'source' position to the target position.

All of the above steps must be performed without ultimately changing any piece
in a previously solved 'active' position or any piece in a 'done' position.

### Step Definition

Thus, a step is defined by:
1. A partition of the puzzle pieces
2. A way to move a piece from any 'pool' position to any other 'pool'
   position of the same parity (without ultimately changing any piece in an
   'active' or 'done' position)
3. For each 'active' position:
   - An index indicating when to solve this position
   - A convenient source 'pool' position
   - A list of moves that will move a piece from the specified 'pool' position
     to this position (without ultimately changing any piece in an 'active'
     position with a lower index or any piece in a 'done' position)

### Stage 1: Solving the outer shell

In this stage, we'll [solve everything but the center 5×5×5][after-stage-1].

We'll accomplish this with 12 steps, each step solving a face of the unsolved
portion of the 9×9×9. We could go in any order; the implementation follows this
ordering (as it looked nice):
1. The `Z=0` plane ([after][after-stage1-step1])
2. The `Z=1` plane ([after][after-stage1-step2])
3. The `Z=8` plane ([after][after-stage1-step3])
4. The `Z=7` plane ([after][after-stage1-step4])
5. The `Y=0` plane ([after][after-stage1-step5])
6. The `Y=1` plane ([after][after-stage1-step6])
7. The `Y=8` plane ([after][after-stage1-step7])
8. ...
8. ...
8. ...
8. ...
12. The `X=7` plane ([after][after-stage-1])

### Stage 1, Step 1: Solving the `Z=0` face

All positions with `Z=0` are 'active', and all other positions are 'pool'.

The most interesting part of this step involves the 'active' positions.  The
'active' positions fall into 3 categories based on their location:
```
A A A A A A A A B B
A A A A A A A A B B
A A A A A A A A B B
A A A A A A A A B B
A A A A A A A A B B
A A A A A A A A B B
A A A A A A A A B B
B B B B B B B B C C
B B B B B B B B C C
```

We'll solve each 'active' position from left-to-right, then top-to-bottom.

The examples that follow focus solely on the moves after the piece is moved
into the 'source' position.

`A` positions are the easiest to solve, as they only require one rotation;
here's an [example for one `A` position][stage1-step1-A].

`B` positions are a little more complicated. Because they're near the
edge, we can't use the same technique as in `A`. This means that we'll have to
move some of the pieces we've previously solved, but we also need to make sure
to restore them to the correct position afterwards. Here's an [example
of how this is accomplished for one `B` position][stage1-step1-B].

If you've played with puzzle cubes before, the technique used in that algorithm
may be familiar. Essentially, we're rotating the target position to a more
convenient place, moving the piece in, and then undoing the rotation of the
target position.

Last and certainly not least, we have `C` positions, which are surrounded on all
sides. Solving them is similar to `B` positions, but a little more complicated.
Here's an [example for one `C` position][stage1-step1-C].

Moving pieces within the pool is rather simple, as there's a lot of room, and the
pool is a rectangular prism; here is an [example for one case][stage1-step1-pool-example]. A full
explanation of the algorithm is omitted, as it is not very novel.

### Stage 1, Steps 2-12

These steps follow the same pattern as Step 1, just on different faces within
the cube.

### Stage 2: Solving the inner shell

At this point, [everything is solved except for the inner 5×5×5][after-stage-1].
After this stage, [everything except the inner 3×3×3 will be solved][after-stage2].

This stage is not part of Stage 1 because some of the algorithms used in Stage 1
require more space than is available in the 5×5×5.

This stage consists of a single step.

All positions outside of in center 5×5×5 are 'done' and will not be changed
(even temporarily). Positions in the center 3×3×3 are 'pool', and the remainder
are 'active'.

### Stage 2, Moving between 'pool' positions

Moving pieces between 'pool' positions is more complex in this step, because of
the limited space. To describe how pieces are moved, we'll first divide the pool
into 4 sections, pictured below.
```
P T P  T Q T  P T P
T Q T  Q U Q  T Q T
P T P  T Q T  P T P
```

`P` positions are the corners, `Q` positions are the face centers, `T` positions
are the edges, and the singular `U` position is the cube center.

`P` and `Q` positions all have odd parity, and `T` and `U` positions all have
even. Thus, pieces can be moved freely between `P` and `Q` positions, and `T` and
`U` positions, but not between `P` and `T` positions, `P` and `U` positions,
etc.

Thus, there are 8 possible cases:
- P: From a `P` position to another `P` position
- Q: From a `Q` position to another `Q` position
- T: From a `T` position to another `T` position
- U: From the `U` position to itself
- PQ: From a `P` position to a `Q` position
- QP: From a `Q` position to a `P` position
- TU: From a `T` position to the `U` position
- UT: From the `U` position to a `T` position

Case U is very trivial, since it is accomplished by doing nothing. Cases P, Q,
T are fairly trivial, and can be accomplished simply by rotating the center
3×3×3.

Cases PQ, QP, TU, UT are where things get interesting — remember that these
must be accomplished without ultimately changing any piece outside of the pool,
the center 3×3×3.

Cases QP & UT are the reverse of cases PQ & TU, respectively, so we will focus
on the later two cases.

Cases PQ & TU are both based off of the following algorithm that we'll call
`INTO_CENTER`, which moves a piece from position `X` into position `U` whilst
preserving everything outside of the center 3×3×3.
```
· · ·  · · ·  · · ·
· · ·  · U ·  · · ·
· · ·  X · ·  · · ·
```

Here is [`INTO_CENTER` in action][INTO_CENTER-demo].

First, Case TU. We can accomplish it with the follwing procedure:
1. Move the piece from the original `T` position to position `V` using Case T.
2. Use `INTO_CENTER` to move the piece from position `V` to position `U`.
```
· · ·  · · ·  · T ·
· · ·  · U ·  · · ·
· · ·  V · ·  · · ·
```

Onto Case PQ. If you look at the moves in `INTO_CENTER` (linked above), you
might notice two things:
1. All of the moves rotate around the `Z` axis
2. All of the moves are at `Z=4`

These two properties mean that `INTO_CENTER` has the same effect on the `Z=3`
and `Z=5` layers as it does on the `Z=4` layer we focused on.

This means that we can accomplish Case PQ with the following procedure:
1. Move the piece from the original `P` position to position `R`, using Case P.
2. Use `INTO_CENTER` to move the piece from position `R` to position `S`.
3. Move the piece from position `S` to the destination `Q` position, using Case
   Q.
```
· · ·  · · ·  · · P
· S ·  · · ·  · · ·
R · ·  · Q ·  · · ·
```

### Stage 2, Moving pieces to 'active' positions

Now that we have a method to move pieces between 'pool' positions, we can move
on to describing the 'active' positions.

First, we'll break up the 'active' positions into a few categories, and then
we'll describe each category.

We'll label the categories `A`-`F`; the partition is pictured below.
```
A B C B A  B D E D B  C E F E C  B D E D B  A B C B A
B D E D B  D · · · D  E · · · E  D · · · D  B D E D B
C E F E C  E · · · E  F · · · F  E · · · E  C E F E C
B D E D B  D · · · D  E · · · E  D · · · D  B D E D B
A B C B A  B D E D B  C E F E C  B D E D B  A B C B A
```

The categories will be solved in alphabetical order; all `A` positions are
solved, then all `B` positions, etc.

For each category, we'll focus on the behavior of only one position; all of the
others follow by symmetry. Specifically, we'll focus on the following positions:
```
A B C · ·  · · · · ·  · · · · ·  · · · · ·  · · · · ·
· D E · ·  · · · · ·  · · · · ·  · · · · ·  · · · · ·
· · F · ·  · · · · ·  · · · · ·  · · · · ·  · · · · ·
· · · · ·  · · · · ·  · · · · ·  · · · · ·  · · · · ·
· · · · ·  · · · · ·  · · · · ·  · · · · ·  · · · · ·
```

When solving a position, care must be taken to not ultimately change any other
position that is outside of the 5×5×5 or in an earlier or current category.

The below diagrams are maps of the 5×5×5. `#` represents a position that must be
ultimately unchanged, `+` represents a 'pool' position, and `X` is the 'pool'
position that we'll later establish as our source 'pool' position.

First up are `A` positions.
```
A · · · #  · · · · ·  · · · · ·  · · · · ·  # · · · #
· · · · ·  · + + + ·  · + + + ·  · + + + ·  · · · · ·
· · · · ·  · + + + ·  · + X + ·  · + + + ·  · · · · ·
· · · · ·  · + + + ·  · + + + ·  · + + + ·  · · · · ·
# · · · #  · · · · ·  · · · · ·  · · · · ·  # · · · #
```
Solving this position is rather trivial. We'll set our source 'pool' position to
be in the center, and to solve the `A` position we'll simply [rotate the piece in][stage2-step1-A].

Let's move on to `B` positions.
```
# B · # #  # · · · #  · · · · ·  # · · · #  # # · # #
# · · · #  · + + + ·  · + + + ·  · + + + ·  # · · · #
· · · · ·  · + + + ·  · + + + ·  · + + + ·  · · · · ·
# · · · #  · + + + ·  · + X + ·  · + + + ·  # · · · #
# # · # #  # · · · #  · · · · ·  # · · · #  # # · # #
```
Here, there are more locked-in pieces, meaning that we can't just simply rotate
the piece in. Thus, we'll have to do [a little bit of maneuvering][stage2-step1-B].

It's getting a bit tighter, but [`C` positions are rather similar to `B`
positions][stage2-step1-C].
```
# # C # #  # · · · #  # · · · #  # · · · #  # # # # #
# · · · #  · + + + ·  · + + + ·  · + + + ·  # · · · #
# · · · #  · + + + ·  · + X + ·  · + + + ·  # · · · #
# · · · #  · + + + ·  · + + + ·  · + + + ·  # · · · #
# # # # #  # · · · #  # · · · #  # · · · #  # # # # #
```

Onto [`D` positions][stage2-step1-D].
```
# # # # #  # # · # #  # · · · #  # # · # #  # # # # #
# D · # #  # + + + #  · + + + ·  # + + + #  # # · # #
# · · · #  · + + + ·  · + + + ·  · + + + ·  # · · · #
# # · # #  # + + + #  · + + + ·  # + + X #  # # · # #
# # # # #  # # · # #  # · · · #  # # · # #  # # # # #
```

It's getting cramped! Time for [`E` positions][stage2-step1-E].
```
# # # # #  # # # # #  # # · # #  # # # # #  # # # # #
# # E # #  # + + + #  # + + + #  # + + + #  # # # # #
# # · # #  # + + + #  · X + + ·  # + + + #  # # · # #
# # # # #  # + + + #  # + + + #  # + + + #  # # # # #
# # # # #  # # # # #  # # · # #  # # # # #  # # # # #
```

Finally, `F` positions. There is almost no room to work with here, but we can
[squeeze the piece in by rotating the corner cube and then one of the edge
cubes][stage2-step1-F].
```
# # # # #  # # # # #  # # # # #  # # # # #  # # # # #
# # # # #  # + + + #  # + + + #  # + + + #  # # # # #
# # F # #  # + + + #  # + + + #  # + + + #  # # # # #
# # # # #  # + + + #  # + + + #  # + X + #  # # # # #
# # # # #  # # # # #  # # # # #  # # # # #  # # # # #
```

### Stage 3: Solving the center 3×3×3

At this point, [everything but the center 3×3×3 is solved][after-stage2]. At the
end of this stage, it will be [entirely solved][solved].

Stage 3 will be broken into 3 steps:
1. Solving [the center cross][stage3-step1-pieces]
2. Solving [the edges][stage3-step2-pieces]
3. Solving [the corners][stage3-step3-pieces]

### Thin Moves

Before moving on to Step 1, we're going to introduce a very useful set of
algorithms: thin moves. These allow us to rotate "flat" 1×3×3 sections of the
cube, without affecting other pieces.

In subsequent examples, thin moves are shown as lines labeled with a `t` — this
is a shorthand for the sequence of regular moves that comprise the thin move.
Here's an example of [the shorthand for a quarter-turn rotation on
Z][thin-move-example].

Here are [the moves that comprise the previous thin move][thin-move-dissection].

<!-- possibly add friendly text -->

### Stage 3, Step 1: Solving the center cross

Again, the center cross is [this set of pieces][stage3-step1-pieces].

As a precursor, we're going to solve the center-most piece first, because we can
simply use the 'pool'-to-'pool' algorithm from Stage 2, Step 1. Afterwards,
[everything but the outer shell of the 3×3×3 will be solved][stage3-step1-done] and we
can proceed to the novel parts of this step.

The partition for this step is as follows:
- ['active'][stage3-step1-active]: all pieces in the center cross other than the center
  piece that we just solved.
- ['done'][stage3-step1-done]: all pieces outside of the center 3×3×3, along with the
  center piece.
- ['pool'][stage3-step1-pool]: all of the corners and edges.

Moving pieces between 'pool' positions is rather simple. We can use thin moves
to rotate the corners and edges of a face of the 3×3×3 without affecting the
center cross. Here's an example of [moving one of the
corners][stage3-step1-pool-example].

To move pieces from a 'pool' position to an 'active' position, we use [a
variation of `INTO_CENTER` that uses thin moves][stage3-step1-pool-to-active].

### Similarity to a Rubik's Cube

At this point, [all positions except the corners and edges of the 3×3×3 are
solved][after-stage3-step1].

The center 3×3×3 is now rather similar to a Rubik's Cube:
- The center cross is solved, and the unsolved pieces are either edges or
  corners; in a Rubik's Cube, the center cross is fixed and is therefore always
  solved.
- Because corners and edges are of different parities, corners cannot swap with
  edges. Likewise, in a Rubik's Cube, corners cannot swap with edges.
- We can either use thin moves to rotate 1×3×3 slices, or regular moves to
  rotate the whole 3×3×3. These are the same moves you can make in a Rubik's
  Cube.

In fact, the rest of the puzzle is simpler than a Rubik's Cube:
- In a Rubik's Cube, every piece in unique, but in this, some of the pieces are
  interchangable.
- In a Rubik's Cube, corners can be in one of three orientations, and edges can
  be in one of two; in this, pieces don't have orientations.

To solve the rest of the cube, we'll use thin moves to solve it like a Rubik's
Cube, using the Old Pochmann ([1][old-pochmann-1], [2][old-pochmann-2]) method.

### Stage 3, Step 2

In this step, the remainder of the cube will be divided as follows, where `A`
and `B` represent 'active' positions, and `C` represents the singular 'pool'
position.
```
· A ·   A · A   · A ·
B · C   · · ·   A · A
· A ·   A · A   · A ·
```

First, we'll solve all `A` positions. Then, we'll solve the `B` position.
Nothing needs to be done to solve the `C` position, as all other pieces of the
same parity will have been solved.

We'll focus on this `A` position; other `A` positions are solved similarly:
```
· · ·   · · ·   · · ·
· · ·   · · ·   · · ·
· A ·   · · ·   · · ·
```

Since the `C` position is the only 'pool' position, our source position will
always be `C`. 

To solve this `A` position, we'll execute the following procedure:
1. Use thin moves to [rotate the piece in this `A` position to the `B` position][stage3-step1-A].
2. Swap the pieces in the `B` and `C` positions.
3. Reverse the moves in #1.

To accomplish \#2, we'll use an algortihm known as the T Permutation, which
swaps the pieces in the `B` and `C` positions, and swaps the pieces in the `P`
and `Q` positions:
```
· · P   · · ·   · · ·
B · C   · · ·   · · ·
· · Q   · · ·   · · ·
```

Thus, the above procedure will swap the pieces in the `B` and `C` positions and
swap the pieces in the `P` and `Q` positions. Note that we must be careful not
to disturb the pieces in the `P` and `Q` positions in #1, as otherwise #2 will
have undesirable effects on the rest of the cube.

As we solve all of the `B` positions, the pieces in the `P` and `Q` positions
keep swapping back and forth, but we don't care, as they are both unsolved. 

Finally, to solve the `B` and `C` positions, if they're not already solved, we'll
apply the T Permutation to swap them.

### Stage 3, Step 3

This step is rather similar to Step 2.

Note that the `P` and `Q` positions have already been solved, and are the same
value when solved. Thus, we don't care if `P` and `Q` are ultimately swapped.

In this step, the remainder of the cube will be divided as follows, where `A`
and `B` represent 'active' positions, and `C` represents the singular 'pool'
position.
```
C · A   · · ·   A · A
· · ·   · · ·   · · ·
A · B   · · ·   A · A
```

Like the previous step, we'll first solve all `A` positions, and then solve the
`B` position. Then, the `C` position must be solved, as all other pieces of the
same parity will have been solved.

We'll focus on this `A` position; other `A` positions are solved similarly:
```
· · ·   · · ·   · · ·
· · ·   · · ·   · · ·
A · ·   · · ·   · · ·
```
To solve this `A` position, we'll execute the following procedure:
1. Use thin moves to rotate the piece in this `A` position to the `B` position.
2. Swap the pieces in the `B` and `C` positions.
3. Reverse the moves in #1.

To accomplish #2, we'll use an algorithm known as the Y Permutation, which swaps
the pieces in the`B` and `C` positions, and swaps the pieces in the `P` and `Q`
positions: 
```
C P ·   · · ·   · · ·
Q · ·   · · ·   · · ·
· · B   · · ·   · · ·
```

Thus, the above procedure will swap the pieces in the `B` and `C` positions and
swap the pieces in the `P` and `Q` positions.

Finally, to solve the `B` and `C` positions, if they're not already solved, we'll
apply the Y Permutation to swap them.

The puzzle is now [entirely solved][solved].

### Conclusion

[We did it!](https://tjjfvi.github.io/cubular)




































[solved]:
https://tjjfvi.github.io/cubular/#solved

[after-stage-1]:
https://tjjfvi.github.io/cubular/#after-stage-1

[after-stage1-step1]:
https://tjjfvi.github.io/cubular/#after-stage1-step1

[after-stage1-step2]:
https://tjjfvi.github.io/cubular/#after-stage1-step2

[after-stage1-step3]:
https://tjjfvi.github.io/cubular/#after-stage1-step3

[after-stage1-step4]:
https://tjjfvi.github.io/cubular/#after-stage1-step4

[after-stage1-step5]:
https://tjjfvi.github.io/cubular/#after-stage1-step5

[after-stage1-step6]:
https://tjjfvi.github.io/cubular/#after-stage1-step6

[after-stage1-step7]:
https://tjjfvi.github.io/cubular/#after-stage1-step7

[stage1-step1-pool-example]:
https://tjjfvi.github.io/cubular/#stage1-step1-pool-example

[stage1-step1-A]:
https://tjjfvi.github.io/cubular/#stage1-step1-A

[stage1-step1-B]:
https://tjjfvi.github.io/cubular/#stage1-step1-B

[stage1-step1-C]:
https://tjjfvi.github.io/cubular/#stage1-step1-C

[INTO_CENTER-demo]:
https://tjjfvi.github.io/cubular/#INTO_CENTER-demo

[stage2-step1-A]:
https://tjjfvi.github.io/cubular/#stage2-step1-A

[stage2-step1-B]:
https://tjjfvi.github.io/cubular/#stage2-step1-B

[stage2-step1-C]:
https://tjjfvi.github.io/cubular/#stage2-step1-C

[stage2-step1-D]:
https://tjjfvi.github.io/cubular/#stage2-step1-D

[stage2-step1-E]:
https://tjjfvi.github.io/cubular/#stage2-step1-E

[stage2-step1-F]:
https://tjjfvi.github.io/cubular/#stage2-step1-F

[after-stage2]:
https://tjjfvi.github.io/cubular/#after-stage2

[stage3-step1-pieces]:
https://tjjfvi.github.io/cubular/#stage3-step1-pieces

[stage3-step2-pieces]:
https://tjjfvi.github.io/cubular/#stage3-step2-pieces

[stage3-step3-pieces]:
https://tjjfvi.github.io/cubular/#stage3-step3-pieces

[thin-move-dissection]:
https://tjjfvi.github.io/cubular/#thin-move-dissection

[thin-move-example]:
https://tjjfvi.github.io/cubular/#thin-move-example

[stage3-step1-done]:
https://tjjfvi.github.io/cubular/#stage3-step1-done

[stage3-step1-active]:
https://tjjfvi.github.io/cubular/#stage3-step1-active

[stage3-step1-pool]:
https://tjjfvi.github.io/cubular/#stage3-step1-pool

[stage3-step1-pool-example]:
https://tjjfvi.github.io/cubular/#stage3-step1-pool-example

[stage3-step1-pool-to-active]:
https://tjjfvi.github.io/cubular/#stage3-step1-pool-to-active

[old-pochmann-1]:
https://www.speedsolving.com/wiki/index.php/Classic_Pochmann

[old-pochmann-2]:
https://ruwix.com/the-rubiks-cube/how-to-solve-the-rubiks-cube-blindfolded-tutorial/

[after-stage3-step1]:
https://tjjfvi.github.io/cubular/#after-stage3-step1

[stage3-step1-A]:
https://tjjfvi.github.io/cubular/#stage3-step1-A
