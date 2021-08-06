
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
of a small algorithm][s1-s1-a].

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
1. The `Z=0` plane ([after][a-s1-s1])
2. The `Z=1` plane ([after][a-s1-s2])
3. The `Z=8` plane ([after][a-s1-s3])
4. The `Z=7` plane ([after][a-s1-s4])
5. The `Y=0` plane ([after][a-s1-s5])
6. The `Y=1` plane ([after][a-s1-s6])
7. The `Y=8` plane ([after][a-s1-s7])
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
here's an [example for one `A` position][s1-s1-a].

`B` positions are a little more complicated. Because they're near the
edge, we can't use the same technique as in `A`. This means that we'll have to
move some of the pieces we've previously solved, but we also need to make sure
to restore them to the correct position afterwards. Here's an [example
of how this is accomplished for one `B` position][s1-s1-b].

If you've played with puzzle cubes before, the technique used in that algorithm
may be familiar. Essentially, we're rotating the target position to a more
convenient place, moving the piece in, and then undoing the rotation of the
target position.

Last and certainly not least, we have `C` positions, which are surrounded on all
sides. Solving them is similar to `B` positions, but a little more complicated.
Here's an [example for one `C` position][s1-s1-c].

Moving pieces within the pool is rather simple, as there's a lot of room, and the
pool is a rectangular prism; here is an [example for one case][s1-s1-pool-example]. A full
explanation of the algorithm is omitted, as it is not very novel.

### Stage 1, Steps 2-12

These steps follow the same pattern as Step 1, just on different faces within
the cube.

### Stage 2: Solving the inner shell

At this point, [everything is solved except for the inner 5×5×5][after-stage-1].
After this stage, [everything except the inner 3×3×3 will be solved][b-s3].

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
be in the center, and to solve the `A` position we'll simply [rotate the piece in][s2-s1-A].

Let's move on to `B` positions.
```
# B · # #  # · · · #  · · · · ·  # · · · #  # # · # #
# · · · #  · + + + ·  · + + + ·  · + + + ·  # · · · #
· · · · ·  · + + + ·  · + + + ·  · + + + ·  · · · · ·
# · · · #  · + + + ·  · + X + ·  · + + + ·  # · · · #
# # · # #  # · · · #  · · · · ·  # · · · #  # # · # #
```
Here, there are more locked-in pieces, meaning that we can't just simply rotate
the piece in. Thus, we'll have to do [a little bit of maneuvering][s2-s1-B].

It's getting a bit tighter, but [`C` positions are rather similar to `B`
positions][s2-s1-C].
```
# # C # #  # · · · #  # · · · #  # · · · #  # # # # #
# · · · #  · + + + ·  · + + + ·  · + + + ·  # · · · #
# · · · #  · + + + ·  · + X + ·  · + + + ·  # · · · #
# · · · #  · + + + ·  · + + + ·  · + + + ·  # · · · #
# # # # #  # · · · #  # · · · #  # · · · #  # # # # #
```

Onto [`D` positions][s2-s1-D].
```
# # # # #  # # · # #  # · · · #  # # · # #  # # # # #
# D · # #  # + + + #  · + + + ·  # + + + #  # # · # #
# · · · #  · + + + ·  · + + + ·  · + + + ·  # · · · #
# # · # #  # + + + #  · + + + ·  # + + X #  # # · # #
# # # # #  # # · # #  # · · · #  # # · # #  # # # # #
```

It's getting cramped! Time for [`E` positions][s2-s1-E].
```
# # # # #  # # # # #  # # · # #  # # # # #  # # # # #
# # E # #  # + + + #  # + + + #  # + + + #  # # # # #
# # · # #  # + + + #  · X + + ·  # + + + #  # # · # #
# # # # #  # + + + #  # + + + #  # + + + #  # # # # #
# # # # #  # # # # #  # # · # #  # # # # #  # # # # #
```

Finally, `F` positions. There is almost no room to work with here, but we can
[squeeze the piece in by rotating the corner cube and then one of the edge
cubes][s2-s1-F].
```
# # # # #  # # # # #  # # # # #  # # # # #  # # # # #
# # # # #  # + + + #  # + + + #  # + + + #  # # # # #
# # F # #  # + + + #  # + + + #  # + + + #  # # # # #
# # # # #  # + + + #  # + + + #  # + X + #  # # # # #
# # # # #  # # # # #  # # # # #  # # # # #  # # # # #
```

### Stage 3: Solving the center 3×3×3

At this point, [everything but the center 3×3×3 is solved][b-s3]. At the
end of this stage, it will be [entirely solved][solved].

Stage 3 will be broken into 3 steps:
1. Solving [the center cross][s3-s1-pieces]
2. Solving [the edges][s3-s2-pieces]
3. Solving [the corners][s3-s3-pieces]

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

Again, the center cross is [this set of pieces][s3-s1-pieces].

As a precursor, we're going to solve the center-most piece first, because we can
simply use the 'pool'-to-'pool' algorithm from Stage 2, Step 1. Afterwards,
[everything but the outer shell of the 3×3×3 will be solved][s3-s1-done] and we
can proceed to the novel parts of this step.

The partition for this step is as follows:
- ['active'][s3-s1-active]: all pieces in the center cross other than the center
  piece that we just solved.
- ['done'][s3-s1-done]: all pieces outside of the center 3×3×3, along with the
  center piece.
- ['pool'][s3-s1-pool]: all of the corners and edges.

Moving pieces between 'pool' positions is rather simple. We can use thin moves
to rotate the corners and edges of a face of the 3×3×3 without affecting the
center cross. Here's an example of [moving one of the
corners][s3-s1-pool-example].

To move pieces from a 'pool' position to an 'active' position, we use [a
variation of `INTO_CENTER` that uses thin moves][s3-s1-pool-to-active].

### Similarity to a Rubik's Cube

At this point, [all positions except the corners and edges of the 3×3×3 are
solved][a-s3-s1].

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
1. Use thin moves to [rotate the piece in this `A` position to the `B` position][s3-s1-a].
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
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678012345780123456801234567012345678123456780234567801456780123567801234678012345780123456801234567012345678123456780234567801345678012567801234678012345780123456801234567012345678123456780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456

[after-stage-1]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012457854523560141234675432345786347456805214167012345678123456780345678012456780123568366234675482145782120056805277567010725478123456780234567801456780123567801234673236145784563056805030567018328278125432180234567801345678012567801234678012345784363456801012867014343478125252180234371601345678012456780123678012345780123456807286167014808678127630580230173401343216312456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-@000@001@002@003@004@005@006@007@008@010@011@012@013@014@015@016@017@018@020@021@022@023@024@025@026@027@028@030@031@032@033@034@035@036@037@038@040@041@042@043@044@045@046@047@048@050@051@052@053@054@055@056@057@058@060@061@062@063@064@065@066@067@068@070@071@072@073@074@075@076@077@078@080@081@082@083@084@085@086@087@088@100@101@102@103@104@105@106@107@108@110@111@112@113@114@115@116@117@118@120@121@122@123@124@125@126@127@128@130@131@132@133@134@135@136@137@138@140@141@142@143@144@145@146@147@148@150@151@152@153@154@155@156@157@158@160@161@162@163@164@165@166@167@168@170@171@172@173@174@175@176@177@178@180@181@182@183@184@185@186@187@188@200@201@202@203@204@205@206@207@208@210@211@212@213@214@215@216@217@218@220@221@227@228@230@231@237@238@240@241@247@248@250@251@257@258@260@261@267@268@270@271@272@273@274@275@276@277@278@280@281@282@283@284@285@286@287@288@300@301@302@303@304@305@306@307@308@310@311@312@313@314@315@316@317@318@320@321@327@328@330@331@337@338@340@341@347@348@350@351@357@358@360@361@367@368@370@371@372@373@374@375@376@377@378@380@381@382@383@384@385@386@387@388@400@401@402@403@404@405@406@407@408@410@411@412@413@414@415@416@417@418@420@421@427@428@430@431@437@438@440@441@447@448@450@451@457@458@460@461@467@468@470@471@472@473@474@475@476@477@478@480@481@482@483@484@485@486@487@488@500@501@502@503@504@505@506@507@508@510@511@512@513@514@515@516@517@518@520@521@527@528@530@531@537@538@540@541@547@548@550@551@557@558@560@561@567@568@570@571@572@573@574@575@576@577@578@580@581@582@583@584@585@586@587@588@600@601@602@603@604@605@606@607@608@610@611@612@613@614@615@616@617@618@620@621@627@628@630@631@637@638@640@641@647@648@650@651@657@658@660@661@667@668@670@671@672@673@674@675@676@677@678@680@681@682@683@684@685@686@687@688@700@701@702@703@704@705@706@707@708@710@711@712@713@714@715@716@717@718@720@721@722@723@724@725@726@727@728@730@731@732@733@734@735@736@737@738@740@741@742@743@744@745@746@747@748@750@751@752@753@754@755@756@757@758@760@761@762@763@764@765@766@767@768@770@771@772@773@774@775@776@777@778@780@781@782@783@784@785@786@787@788@800@801@802@803@804@805@806@807@808@810@811@812@813@814@815@816@817@818@820@821@822@823@824@825@826@827@828@830@831@832@833@834@835@836@837@838@840@841@842@843@844@845@846@847@848@850@851@852@853@854@855@856@857@858@860@861@862@863@864@865@866@867@868@870@871@872@873@874@875@876@877@878@880@881@882@883@884@885@886@887@888

[a-s1-s1]:
https://tjjfvi.github.io/cubular/#045262845187521651238406341352371481473684833535711254661364124702324486868774171117307256221258261311528638403451724570088657627587105733067732807270085000024043223156305353455418443518581512024686653051543778246556845632173056302428128476141304666650424722200558588018607005621754208252873614747076545821161130464266762425441811013504365114685214564738148502887657611077571735105676346263304832301438727550886837688776257704126066822282103034306616163852838236807453363534127423366121620416367788753518871685367080815617176636706253247311340474248472101272545457478704007273821108414002341284125753040288315034380750143433555164563860507632863538801232807072077842126002765210028888305408307427847016583554846610205074783733617-@000@010@020@030@040@050@060@070@080@100@110@120@130@140@150@160@170@180@200@210@220@230@240@250@260@270@280@300@310@320@330@340@350@360@370@380@400@410@420@430@440@450@460@470@480@500@510@520@530@540@550@560@570@580@600@610@620@630@640@650@660@670@680@700@710@720@730@740@750@760@770@780@800@810@820@830@840@850@860@870@880

[a-s1-s2]:
https://tjjfvi.github.io/cubular/#018270845122723651238146341341125481455728833566078254671637124786331686801184571122503256230411261344862638455516724567047657677074105788767832803852085010278643235431305341243418451534581564003686671827543784320556800714773014887128123333641342100650455752200566480018675782621785581252801260747018543821122630464238365425455776013562745114671608564786043502806606611014087735121264346230513832346857727568148837673672257782118066801880103016347616123404838230703453345233127456507121670754367782525518800080367014120617125473307238685013342077472450528532563658618782520273800788414014343284120653040232548030345610141455325274567615507676357078801662807018026842123812765232742888345004746456325631567070818674327464780433647-@000@001@010@011@020@021@030@031@040@041@050@051@060@061@070@071@080@081@100@101@110@111@120@121@130@131@140@141@150@151@160@161@170@171@180@181@200@201@210@211@220@221@230@231@240@241@250@251@260@261@270@271@280@281@300@301@310@311@320@321@330@331@340@341@350@351@360@361@370@371@380@381@400@401@410@411@420@421@430@431@440@441@450@451@460@461@470@471@480@481@500@501@510@511@520@521@530@531@540@541@550@551@560@561@570@571@580@581@600@601@610@611@620@621@630@631@640@641@650@651@660@661@670@671@680@681@700@701@710@711@720@721@730@731@740@741@750@751@760@761@770@771@780@781@800@801@810@811@820@821@830@831@840@841@850@851@860@861@870@871@880@881

[a-s1-s3]:
https://tjjfvi.github.io/cubular/#018274628122701470238152341341128672455724713566085244671652725786371466801104447122562850230416321344831412455554863567763254677485505788306486803832577010241488235474571341271052451511553564081454671762125784047376800634877014815678123350280342182232455742743566403034675784135785501216801665557018680778122630320238364031455686183562305054671500185786067046806603147014043248121210700230580201346852382568167614673437315782378086801882087016385838123454820230748181345272812456537433670084335782371766800027547014143608125468300238628061342012522450565223563676114782303026800666007014847388120677100232524361345672872455381133567630744676325685801811667018777458123214520232763571345051372456364283567036714674357835780417886-@000@001@008@010@011@018@020@021@028@030@031@038@040@041@048@050@051@058@060@061@068@070@071@078@080@081@088@100@101@108@110@111@118@120@121@128@130@131@138@140@141@148@150@151@158@160@161@168@170@171@178@180@181@188@200@201@208@210@211@218@220@221@228@230@231@238@240@241@248@250@251@258@260@261@268@270@271@278@280@281@288@300@301@308@310@311@318@320@321@328@330@331@338@340@341@348@350@351@358@360@361@368@370@371@378@380@381@388@400@401@408@410@411@418@420@421@428@430@431@438@440@441@448@450@451@458@460@461@468@470@471@478@480@481@488@500@501@508@510@511@518@520@521@528@530@531@538@540@541@548@550@551@558@560@561@568@570@571@578@580@581@588@600@601@608@610@611@618@620@621@628@630@631@638@640@641@648@650@651@658@660@661@668@670@671@678@680@681@688@700@701@708@710@711@718@720@721@728@730@731@738@740@741@748@750@751@758@760@761@768@770@771@778@780@781@788@800@801@808@810@811@818@820@821@828@830@831@838@840@841@848@850@851@858@860@861@868@870@871@878@880@881@888

[a-s1-s4]:
https://tjjfvi.github.io/cubular/#018487578122358180238046101341351012455812723566328434671650545786505456801484767122773080230650301344621712455454423567465634677778145788588856803044567010035278231415301344088712458412623564173434671087345784530256800632567014376778123482880342177312453066523561321634675688145785266656801651167018025678122423880238786401453214523562361034675272845786386256806465867014481678121220080230641301346152812560318434676222045788101756801472467016347478123206780230763201345633512456327523673252045782775556808011567014313078125765180238243701342286112450307423563480734784181056805486167014321178120132180232481201345670812455768723567038334676705045803077367014523078125817080232887401345675012456673423567070334674320745780258656-@000@001@007@008@010@011@017@018@020@021@027@028@030@031@037@038@040@041@047@048@050@051@057@058@060@061@067@068@070@071@077@078@080@081@087@088@100@101@107@108@110@111@117@118@120@121@127@128@130@131@137@138@140@141@147@148@150@151@157@158@160@161@167@168@170@171@177@178@180@181@187@188@200@201@207@208@210@211@217@218@220@221@227@228@230@231@237@238@240@241@247@248@250@251@257@258@260@261@267@268@270@271@277@278@280@281@287@288@300@301@307@308@310@311@317@318@320@321@327@328@330@331@337@338@340@341@347@348@350@351@357@358@360@361@367@368@370@371@377@378@380@381@387@388@400@401@407@408@410@411@417@418@420@421@427@428@430@431@437@438@440@441@447@448@450@451@457@458@460@461@467@468@470@471@477@478@480@481@487@488@500@501@507@508@510@511@517@518@520@521@527@528@530@531@537@538@540@541@547@548@550@551@557@558@560@561@567@568@570@571@577@578@580@581@587@588@600@601@607@608@610@611@617@618@620@621@627@628@630@631@637@638@640@641@647@648@650@651@657@658@660@661@667@668@670@671@677@678@680@681@687@688@700@701@707@708@710@711@717@718@720@721@727@728@730@731@737@738@740@741@747@748@750@751@757@758@760@761@767@768@770@771@777@778@780@781@787@788@800@801@807@808@810@811@817@818@820@821@827@828@830@831@837@838@840@841@847@848@850@851@857@858@860@861@867@868@870@871@877@878@880@881@887@888

[a-s1-s5]:
https://tjjfvi.github.io/cubular/#012345678123103880235254201346371812455431823564845434676212545784882456801484767123456780235284701347083212453246123560365434675734545780543856805607567010035278234567801341371612451432123561341434671472345782528056803838567014064778123482880345678012451656123568177534675216745782100256801476567018560678122034880238786401456780123561325534677822345782165856807412367014403878121220080230641301346152812567801234671571545786560156805472767012806778120816380230763201345633512456327523678012345786146256801764367012501078123601880234503701342286112450307423563480734780123456807611067010513278127666380238420501347680812455768723567038334676705045801234567017861478121487880234527801345208112456673423567070334674320745780258656-@000@001@002@003@004@005@006@007@008@010@011@017@018@020@021@027@028@030@031@037@038@040@041@047@048@050@051@057@058@060@061@067@068@070@071@077@078@080@081@087@088@100@101@102@103@104@105@106@107@108@110@111@117@118@120@121@127@128@130@131@137@138@140@141@147@148@150@151@157@158@160@161@167@168@170@171@177@178@180@181@187@188@200@201@202@203@204@205@206@207@208@210@211@217@218@220@221@227@228@230@231@237@238@240@241@247@248@250@251@257@258@260@261@267@268@270@271@277@278@280@281@287@288@300@301@302@303@304@305@306@307@308@310@311@317@318@320@321@327@328@330@331@337@338@340@341@347@348@350@351@357@358@360@361@367@368@370@371@377@378@380@381@387@388@400@401@402@403@404@405@406@407@408@410@411@417@418@420@421@427@428@430@431@437@438@440@441@447@448@450@451@457@458@460@461@467@468@470@471@477@478@480@481@487@488@500@501@502@503@504@505@506@507@508@510@511@517@518@520@521@527@528@530@531@537@538@540@541@547@548@550@551@557@558@560@561@567@568@570@571@577@578@580@581@587@588@600@601@602@603@604@605@606@607@608@610@611@617@618@620@621@627@628@630@631@637@638@640@641@647@648@650@651@657@658@660@661@667@668@670@671@677@678@680@681@687@688@700@701@702@703@704@705@706@707@708@710@711@717@718@720@721@727@728@730@731@737@738@740@741@747@748@750@751@757@758@760@761@767@768@770@771@777@778@780@781@787@788@800@801@802@803@804@805@806@807@808@810@811@817@818@820@821@827@828@830@831@837@838@840@841@847@848@850@851@857@858@860@861@867@868@870@871@877@878@880@881@887@888

[a-s1-s6]:
https://tjjfvi.github.io/cubular/#012345678123456780230417101342841012455252123562141634676252145784668456801484767123456780234567801342343812450274823568313434673170545780581456805008567010035278234567801345678012453650123564021134675506145784345756805038167014340778123282880345678012456780123566563434677236545788726056803438567018548778123087880236786401456780123567801234674227245788128256805406567018645678125652880238105601343076512567801234678012345786727056807472367016545878127676780236845001342632712456328223678012345780123456803676167018781278121412380231803401342821512450365023563070234780123456801234567011148678125136080234141701345051012455333623567475634676023345801234567012345678120676080232083801341231312452187723567077134674377145780458456-@000@001@002@003@004@005@006@007@008@010@011@012@013@014@015@016@017@018@020@021@027@028@030@031@037@038@040@041@047@048@050@051@057@058@060@061@067@068@070@071@077@078@080@081@087@088@100@101@102@103@104@105@106@107@108@110@111@112@113@114@115@116@117@118@120@121@127@128@130@131@137@138@140@141@147@148@150@151@157@158@160@161@167@168@170@171@177@178@180@181@187@188@200@201@202@203@204@205@206@207@208@210@211@212@213@214@215@216@217@218@220@221@227@228@230@231@237@238@240@241@247@248@250@251@257@258@260@261@267@268@270@271@277@278@280@281@287@288@300@301@302@303@304@305@306@307@308@310@311@312@313@314@315@316@317@318@320@321@327@328@330@331@337@338@340@341@347@348@350@351@357@358@360@361@367@368@370@371@377@378@380@381@387@388@400@401@402@403@404@405@406@407@408@410@411@412@413@414@415@416@417@418@420@421@427@428@430@431@437@438@440@441@447@448@450@451@457@458@460@461@467@468@470@471@477@478@480@481@487@488@500@501@502@503@504@505@506@507@508@510@511@512@513@514@515@516@517@518@520@521@527@528@530@531@537@538@540@541@547@548@550@551@557@558@560@561@567@568@570@571@577@578@580@581@587@588@600@601@602@603@604@605@606@607@608@610@611@612@613@614@615@616@617@618@620@621@627@628@630@631@637@638@640@641@647@648@650@651@657@658@660@661@667@668@670@671@677@678@680@681@687@688@700@701@702@703@704@705@706@707@708@710@711@712@713@714@715@716@717@718@720@721@727@728@730@731@737@738@740@741@747@748@750@751@757@758@760@761@767@768@770@771@777@778@780@781@787@788@800@801@802@803@804@805@806@807@808@810@811@812@813@814@815@816@817@818@820@821@827@828@830@831@837@838@840@841@847@848@850@851@857@858@860@861@867@868@870@871@877@878@880@881@887@888

[a-s1-s7]:
https://tjjfvi.github.io/cubular/#012345678123456780230417101342841012456232823560645034678277545786785756801234567123456780234567801342343812450274823562857834671214345784105156807654567012345678234567801345678012453650123562151134677414845780573056805751867012103678123456780345678012456780123566563434673656545780125056803400567010127478123232880234567801456780123567801234674227245784548256807457667012345678125238880234523001345678012567801234678012345786727056807172367014827678125414380235521801348850612456780123678012345780123456803676167014781278123460380236101801345838312450383723567801234780123456801234567011148678120116080237523401347636312450076423568076034678012345801234567012345678120676080236123801348456712454382223565808134676741345780123456-@000@001@002@003@004@005@006@007@008@010@011@012@013@014@015@016@017@018@020@021@027@028@030@031@037@038@040@041@047@048@050@051@057@058@060@061@067@068@070@071@077@078@080@081@082@083@084@085@086@087@088@100@101@102@103@104@105@106@107@108@110@111@112@113@114@115@116@117@118@120@121@127@128@130@131@137@138@140@141@147@148@150@151@157@158@160@161@167@168@170@171@177@178@180@181@182@183@184@185@186@187@188@200@201@202@203@204@205@206@207@208@210@211@212@213@214@215@216@217@218@220@221@227@228@230@231@237@238@240@241@247@248@250@251@257@258@260@261@267@268@270@271@277@278@280@281@282@283@284@285@286@287@288@300@301@302@303@304@305@306@307@308@310@311@312@313@314@315@316@317@318@320@321@327@328@330@331@337@338@340@341@347@348@350@351@357@358@360@361@367@368@370@371@377@378@380@381@382@383@384@385@386@387@388@400@401@402@403@404@405@406@407@408@410@411@412@413@414@415@416@417@418@420@421@427@428@430@431@437@438@440@441@447@448@450@451@457@458@460@461@467@468@470@471@477@478@480@481@482@483@484@485@486@487@488@500@501@502@503@504@505@506@507@508@510@511@512@513@514@515@516@517@518@520@521@527@528@530@531@537@538@540@541@547@548@550@551@557@558@560@561@567@568@570@571@577@578@580@581@582@583@584@585@586@587@588@600@601@602@603@604@605@606@607@608@610@611@612@613@614@615@616@617@618@620@621@627@628@630@631@637@638@640@641@647@648@650@651@657@658@660@661@667@668@670@671@677@678@680@681@682@683@684@685@686@687@688@700@701@702@703@704@705@706@707@708@710@711@712@713@714@715@716@717@718@720@721@727@728@730@731@737@738@740@741@747@748@750@751@757@758@760@761@767@768@770@771@777@778@780@781@782@783@784@785@786@787@788@800@801@802@803@804@805@806@807@808@810@811@812@813@814@815@816@817@818@820@821@827@828@830@831@837@838@840@841@847@848@850@851@857@858@860@861@867@868@870@871@877@878@880@881@882@883@884@885@886@887@888

[s1-s1-pool-example]:
https://tjjfvi.github.io/cubular/#002678842141748653215415571316035441445360715552745624643388533756163414825806825104247718277616737331088251411632788505367474628346025786448463806272773017527250281220763312768081477454315518563638623724344740543722862776458064363182107864435308174430425565517527667452617587856774505588861810457055630230162620307258576867443414500576323666620051303702583834871014768012068488161678775215845212345835445511753600640062862718783288820215745077528752183824570274086434343214143417578063635018003746045732876610702034020538165026773251861670321610348456300283513216640712621352863130810017582112181051471246363730345187803410587570556212167680325310811282443014521065123318308250803414320666002474022800581242843662767034783876823-525Z1@615-545Z1@635-565Z1@655-665Z3@675-666X2@765-@767

[s1-s1-a]:
https://tjjfvi.github.io/cubular/#001353526166740834715667458810843654381502348412215436182482657477332235857286883162803036243424123638347082500052711476225212565674766434778817168411586048275112244752705357333012387515475822826277072066813484843007628485503705413684360438868352527220184184551372537356161036777552125480785127563458205856808607331234746008485652008035367164774478463684710134110004503884168633167040160010124785585342536570347286383432361414833470624072388046380260527170443438765016611304315064001234604432721088805615155707720875664082651412651336528717366255121087722746073666140737811007523574107753142257501625543774003221515078820034525302618731223660343872865757747710552862681632162651748646080817131458161244128810335046871465602212088-421Y1@@312-@@310

[s1-s1-b]:
https://tjjfvi.github.io/cubular/#045812325456533373701710762560408055207418750854640804071706557656001557858612861185165412805670078202564012863857007512741014620487766047563601741261320067610004274631763481373744355260825276868071355818164276374260560214750036308050187677778330886414348704337416277173142460348065240480601647487202608646318684572088243010465832327238306613254731336287348566127205824452767836327850506631268576155628333514341411378804141812287220448277375262025857374214413240623811801014188272735750621321640257861471607277781358501880845234051237522426717236825548785652342402011837325264721888305136263863725682537456136785333816541487508454871262163157561348103730155404515033808024482414520631346435235088101231113250754654546834538632367-711Y1@600@@702-712Y3@800@@601-711Y3@800@@801-@600@@700

[s1-s1-c]:
https://tjjfvi.github.io/cubular/#047284171132813215221150543338863007408416653551213434687465276730163036525156466176161535202018862314155584425342830532802482673774784772066832825223860261321475278237526340522244461767641510060756633156346732673576800530107056547084361713245335173452445220805546627656602358155754302271837432013036310231112856341816778432403280004556065855687813021717343472864680323057035786101484167288725076763850177518548754668630560706055714884152144014121428150802548222813511315708130268131651627283366730166164843013573010787850188286845202585887306432643478263438237413707700828270840480081072478434107257381267587612363483785458785242441472888436511652877618560050407066163431004274862210317502126407763055547482447651286078707034541-771X1@660@760@670@@783-771Y1@680@780@681@@783-672Y3@880@881@780@@783-771Y3@880@881@780@@781-771X3@680@780@681@@781-@660@760@670@@770

[INTO_CENTER-demo]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012457754823564501634623634545714123856803030367012345678123456780345678012456780123560547234675067345712548456805402367086508478123456780234567801456780123567801234601274545712848256823437167088341678177632180234567801345678012567801234678012345786321256801632567014528278125654180234701201345678012456780123678012345780123456871230167086565078101238380234723001341234112456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-454Z3@@354@234@244@254@564@464@364-344Z1@@464@234@244@254@544@554@564-454Z1@@464@434@334@234@544@554@564-344Z3@@354@434@334@234@564@464@364-@@444@234@244@254@564@464@364

[s2-s1-A]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012455676523564743434673834345782165256807230867012345678123456780345678012456780123566341234677252145786348856808634167016873478123456780234567801456780123567801234673210545784325756803062567010141778127232380234567801345678012567801234678012345782543056805030567012120078125454380234803601345678012456780123678012345780123456803684167018125478127214180232561201345410112456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-333Z2@@444-333X1@@224-@@222

[s2-s1-B]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456274123560305234673458345786301256801454567012345678123456780345678012456780123564505234673084145786145256803052367016775478123456780234567801456780123567801234673210345788501456807432367010371278123616180234567801345678012567801234678012345782841656803216367014563678121232880234807401345678012456780123678012345780123456801482567016728278127437380232581401345452012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-333Z1@222@@454-333Y1@422@@454-444Z2@424@@454-333Y3@424@@434-333Z3@422@@432-@222@@322

[s2-s1-C]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456710123567545234671034145780507456801254567012345678123456780345678012456780123567303234678234745784325656805456167012161678123456780234567801456780123567801234673432745784121656803280367014303278127272780234567801345678012567801234678012345780565456801432167012541078123630580234363801345678012456780123678012345780123456801284567012843678125282380234341801345688012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-454Z3@@444@222@322@232-333Z1@@354@222@322@232-333Y1@@354@422@432@322-454Z1@@354@424@434@423-333Y3@@444@424@434@423-333Z3@@442@422@432@322-@@422@222@322@232

[s2-s1-D]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567741234678232345780163456801234567012345678123456780345678012456780123567123234671410345782345056805436567012163678123456780234567801456780123567801234678256345784563456801454367012105478123270780234567801345678012567801234678012345780321456805274867010125478125480880234321801345678012456780123678012345780123456801234567012323678123616780234505801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-333Z1@@554@222@322@422@232@242-333Y2@@554@422@432@442@322@222-444Z2@@554@224@234@244@324@424-333Y2@@334@224@234@244@324@424-333Z3@@332@422@432@442@322@222-@@332@222@322@422@232@242

[s2-s1-E]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567861234678432345780123456801234567012345678123456780345678012456780123567841234678214345782305056801430567012305678123456780234567801456780123567801234678256345784541456803630367014121078123256780234567801345678012567801234678012345780143456801450567012325278123416780234547801345678012456780123678012345780123456801234567012325678123616780234527801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-454Z3@@344@222@322@422@232@332@242-333Z3@@364@222@322@422@232@332@242-333X2@@364@242@232@222@342@332@442-454Z1@@364@224@324@424@234@334@244-333X2@@344@224@324@424@234@334@244-333Z1@@322@242@232@222@342@332@442-@@432@222@322@422@232@332@242

[s2-s1-F]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678052345780123456801234567012345678123456780345678012456780123567801234678416345780541456801212567012345678123456780234567801456780123567801234678032345780323456805212167012541678123436780234567801345678012567801234678012345780123456801432567012503678123434780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-333Z1@@455@222@322@422@232@332@432@242@342-333Y2@@455@422@432@442@322@332@342@222@232-554X1@@455@224@324@424@234@334@434@244@344-333Y2@@444@224@324@424@234@334@434@244@344-333Z3@@242@422@432@442@322@332@342@222@232-554X3@@442@222@322@422@232@332@432@242@342-@@442@222@322@422@232@332@432@242@342

[b-s3]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678432345780323456801456567012345678123456780234567801456780123567801234678012345780503456801212567012343678123456780234567801345678012567801234678012345780123456801434567012541678123212780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-@000@001@002@003@004@005@006@007@008@010@011@012@013@014@015@016@017@018@020@021@022@023@024@025@026@027@028@030@031@032@033@034@035@036@037@038@040@041@042@043@044@045@046@047@048@050@051@052@053@054@055@056@057@058@060@061@062@063@064@065@066@067@068@070@071@072@073@074@075@076@077@078@080@081@082@083@084@085@086@087@088@100@101@102@103@104@105@106@107@108@110@111@112@113@114@115@116@117@118@120@121@122@123@124@125@126@127@128@130@131@132@133@134@135@136@137@138@140@141@142@143@144@145@146@147@148@150@151@152@153@154@155@156@157@158@160@161@162@163@164@165@166@167@168@170@171@172@173@174@175@176@177@178@180@181@182@183@184@185@186@187@188@200@201@202@203@204@205@206@207@208@210@211@212@213@214@215@216@217@218@220@221@222@223@224@225@226@227@228@230@231@232@233@234@235@236@237@238@240@241@242@243@244@245@246@247@248@250@251@252@253@254@255@256@257@258@260@261@262@263@264@265@266@267@268@270@271@272@273@274@275@276@277@278@280@281@282@283@284@285@286@287@288@300@301@302@303@304@305@306@307@308@310@311@312@313@314@315@316@317@318@320@321@322@323@324@325@326@327@328@330@331@332@336@337@338@340@341@342@346@347@348@350@351@352@356@357@358@360@361@362@363@364@365@366@367@368@370@371@372@373@374@375@376@377@378@380@381@382@383@384@385@386@387@388@400@401@402@403@404@405@406@407@408@410@411@412@413@414@415@416@417@418@420@421@422@423@424@425@426@427@428@430@431@432@436@437@438@440@441@442@446@447@448@450@451@452@456@457@458@460@461@462@463@464@465@466@467@468@470@471@472@473@474@475@476@477@478@480@481@482@483@484@485@486@487@488@500@501@502@503@504@505@506@507@508@510@511@512@513@514@515@516@517@518@520@521@522@523@524@525@526@527@528@530@531@532@536@537@538@540@541@542@546@547@548@550@551@552@556@557@558@560@561@562@563@564@565@566@567@568@570@571@572@573@574@575@576@577@578@580@581@582@583@584@585@586@587@588@600@601@602@603@604@605@606@607@608@610@611@612@613@614@615@616@617@618@620@621@622@623@624@625@626@627@628@630@631@632@633@634@635@636@637@638@640@641@642@643@644@645@646@647@648@650@651@652@653@654@655@656@657@658@660@661@662@663@664@665@666@667@668@670@671@672@673@674@675@676@677@678@680@681@682@683@684@685@686@687@688@700@701@702@703@704@705@706@707@708@710@711@712@713@714@715@716@717@718@720@721@722@723@724@725@726@727@728@730@731@732@733@734@735@736@737@738@740@741@742@743@744@745@746@747@748@750@751@752@753@754@755@756@757@758@760@761@762@763@764@765@766@767@768@770@771@772@773@774@775@776@777@778@780@781@782@783@784@785@786@787@788@800@801@802@803@804@805@806@807@808@810@811@812@813@814@815@816@817@818@820@821@822@823@824@825@826@827@828@830@831@832@833@834@835@836@837@838@840@841@842@843@844@845@846@847@848@850@851@852@853@854@855@856@857@858@860@861@862@863@864@865@866@867@868@870@871@872@873@874@875@876@877@878@880@881@882@883@884@885@886@887@888

[s3-s1-pieces]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678012345780123456801234567012345678123456780234567801456780123567801234678012345780123456801234567012345678123456780234567801345678012567801234678012345780123456801234567012345678123456780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-@444@344@544@434@454@443@445

[s3-s2-pieces]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678012345780123456801234567012345678123456780234567801456780123567801234678012345780123456801234567012345678123456780234567801345678012567801234678012345780123456801234567012345678123456780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-@343@543@433@453@345@545@435@455@334@354@534@554

[s3-s3-pieces]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678012345780123456801234567012345678123456780234567801456780123567801234678012345780123456801234567012345678123456780234567801345678012567801234678012345780123456801234567012345678123456780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-@333@335@353@355@533@535@553@555

[thin-move-dissection]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678012345780123456801234567012345678123456780234567801456780123567801234678012345780123456801234567012345678123456780234567801345678012567801234678012345780123456801234567012345678123456780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-443Z1@@444@@434-444X2@@444@@544-443Z3@@444@@544-444X2@@444@@434-444Z3@@444@@454-@@444@@544

[thin-move-example]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678012345780123456801234567012345678123456780234567801456780123567801234678012345780123456801234567012345678123456780234567801345678012567801234678012345780123456801234567012345678123456780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-444Z1t@@444@@434-@@444@@544

[s3-s1-done]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678232345780343456801012567012345678123456780234567801456780123567801234678012345780145456801432567012543678123456780234567801345678012567801234678012345780123456801432567012523678123416780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-@000@001@002@003@004@005@006@007@008@010@011@012@013@014@015@016@017@018@020@021@022@023@024@025@026@027@028@030@031@032@033@034@035@036@037@038@040@041@042@043@044@045@046@047@048@050@051@052@053@054@055@056@057@058@060@061@062@063@064@065@066@067@068@070@071@072@073@074@075@076@077@078@080@081@082@083@084@085@086@087@088@100@101@102@103@104@105@106@107@108@110@111@112@113@114@115@116@117@118@120@121@122@123@124@125@126@127@128@130@131@132@133@134@135@136@137@138@140@141@142@143@144@145@146@147@148@150@151@152@153@154@155@156@157@158@160@161@162@163@164@165@166@167@168@170@171@172@173@174@175@176@177@178@180@181@182@183@184@185@186@187@188@200@201@202@203@204@205@206@207@208@210@211@212@213@214@215@216@217@218@220@221@222@223@224@225@226@227@228@230@231@232@233@234@235@236@237@238@240@241@242@243@244@245@246@247@248@250@251@252@253@254@255@256@257@258@260@261@262@263@264@265@266@267@268@270@271@272@273@274@275@276@277@278@280@281@282@283@284@285@286@287@288@300@301@302@303@304@305@306@307@308@310@311@312@313@314@315@316@317@318@320@321@322@323@324@325@326@327@328@330@331@332@336@337@338@340@341@342@346@347@348@350@351@352@356@357@358@360@361@362@363@364@365@366@367@368@370@371@372@373@374@375@376@377@378@380@381@382@383@384@385@386@387@388@400@401@402@403@404@405@406@407@408@410@411@412@413@414@415@416@417@418@420@421@422@423@424@425@426@427@428@430@431@432@436@437@438@440@441@442@446@447@448@450@451@452@456@457@458@460@461@462@463@464@465@466@467@468@470@471@472@473@474@475@476@477@478@480@481@482@483@484@485@486@487@488@500@501@502@503@504@505@506@507@508@510@511@512@513@514@515@516@517@518@520@521@522@523@524@525@526@527@528@530@531@532@536@537@538@540@541@542@546@547@548@550@551@552@556@557@558@560@561@562@563@564@565@566@567@568@570@571@572@573@574@575@576@577@578@580@581@582@583@584@585@586@587@588@600@601@602@603@604@605@606@607@608@610@611@612@613@614@615@616@617@618@620@621@622@623@624@625@626@627@628@630@631@632@633@634@635@636@637@638@640@641@642@643@644@645@646@647@648@650@651@652@653@654@655@656@657@658@660@661@662@663@664@665@666@667@668@670@671@672@673@674@675@676@677@678@680@681@682@683@684@685@686@687@688@700@701@702@703@704@705@706@707@708@710@711@712@713@714@715@716@717@718@720@721@722@723@724@725@726@727@728@730@731@732@733@734@735@736@737@738@740@741@742@743@744@745@746@747@748@750@751@752@753@754@755@756@757@758@760@761@762@763@764@765@766@767@768@770@771@772@773@774@775@776@777@778@780@781@782@783@784@785@786@787@788@800@801@802@803@804@805@806@807@808@810@811@812@813@814@815@816@817@818@820@821@822@823@824@825@826@827@828@830@831@832@833@834@835@836@837@838@840@841@842@843@844@845@846@847@848@850@851@852@853@854@855@856@857@858@860@861@862@863@864@865@866@867@868@870@871@872@873@874@875@876@877@878@880@881@882@883@884@885@886@887@888@444

[s3-s1-active]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678232345780343456801012567012345678123456780234567801456780123567801234678012345780145456801432567012543678123456780234567801345678012567801234678012345780123456801432567012523678123416780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-@344@544@434@454@443@445

[s3-s1-pool]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678232345780343456801012567012345678123456780234567801456780123567801234678012345780145456801432567012543678123456780234567801345678012567801234678012345780123456801432567012523678123416780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-@333@335@353@355@533@535@553@555@343@543@433@453@345@545@435@455@334@354@534@554

[s3-s1-pool-example]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678232345780541456801234567012345678123456780234567801456780123567801234678012345780343456801234567012325678123456780234567801345678012567801234678012345780123456801210567012345678123614780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-443Z3t@@553@443@434-434Y2t@@533@443@434-@@335@443@434

[s3-s1-pool-to-active]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678214345780345456801210567012345678123456780234567801456780123567801234678012345780325456801634567012343678123456780234567801345678012567801234678012345780123456801234567012541678123232780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-433Z1t@@333@323@423@523@233@243@253-343Z3t@@423@523@533@543@233@243@253-433Z3t@@423@523@533@543@253@353@453-343Z1t@@333@323@423@523@253@353@453-@@443@323@423@523@233@243@253

[old-pochmann-1]:
https://www.speedsolving.com/wiki/index.php/Classic_Pochmann

[old-pochmann-2]:
https://ruwix.com/the-rubiks-cube/how-to-solve-the-rubiks-cube-blindfolded-tutorial/

[a-s3-s1]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678432345780323456801456567012345678123456780234567801456780123567801234678012345780503456801212567012343678123456780234567801345678012567801234678012345780123456801434567012541678123212780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-@000@001@002@003@004@005@006@007@008@010@011@012@013@014@015@016@017@018@020@021@022@023@024@025@026@027@028@030@031@032@033@034@035@036@037@038@040@041@042@043@044@045@046@047@048@050@051@052@053@054@055@056@057@058@060@061@062@063@064@065@066@067@068@070@071@072@073@074@075@076@077@078@080@081@082@083@084@085@086@087@088@100@101@102@103@104@105@106@107@108@110@111@112@113@114@115@116@117@118@120@121@122@123@124@125@126@127@128@130@131@132@133@134@135@136@137@138@140@141@142@143@144@145@146@147@148@150@151@152@153@154@155@156@157@158@160@161@162@163@164@165@166@167@168@170@171@172@173@174@175@176@177@178@180@181@182@183@184@185@186@187@188@200@201@202@203@204@205@206@207@208@210@211@212@213@214@215@216@217@218@220@221@222@223@224@225@226@227@228@230@231@232@233@234@235@236@237@238@240@241@242@243@244@245@246@247@248@250@251@252@253@254@255@256@257@258@260@261@262@263@264@265@266@267@268@270@271@272@273@274@275@276@277@278@280@281@282@283@284@285@286@287@288@300@301@302@303@304@305@306@307@308@310@311@312@313@314@315@316@317@318@320@321@322@323@324@325@326@327@328@330@331@332@336@337@338@340@341@342@346@347@348@350@351@352@356@357@358@360@361@362@363@364@365@366@367@368@370@371@372@373@374@375@376@377@378@380@381@382@383@384@385@386@387@388@400@401@402@403@404@405@406@407@408@410@411@412@413@414@415@416@417@418@420@421@422@423@424@425@426@427@428@430@431@432@436@437@438@440@441@442@446@447@448@450@451@452@456@457@458@460@461@462@463@464@465@466@467@468@470@471@472@473@474@475@476@477@478@480@481@482@483@484@485@486@487@488@500@501@502@503@504@505@506@507@508@510@511@512@513@514@515@516@517@518@520@521@522@523@524@525@526@527@528@530@531@532@536@537@538@540@541@542@546@547@548@550@551@552@556@557@558@560@561@562@563@564@565@566@567@568@570@571@572@573@574@575@576@577@578@580@581@582@583@584@585@586@587@588@600@601@602@603@604@605@606@607@608@610@611@612@613@614@615@616@617@618@620@621@622@623@624@625@626@627@628@630@631@632@633@634@635@636@637@638@640@641@642@643@644@645@646@647@648@650@651@652@653@654@655@656@657@658@660@661@662@663@664@665@666@667@668@670@671@672@673@674@675@676@677@678@680@681@682@683@684@685@686@687@688@700@701@702@703@704@705@706@707@708@710@711@712@713@714@715@716@717@718@720@721@722@723@724@725@726@727@728@730@731@732@733@734@735@736@737@738@740@741@742@743@744@745@746@747@748@750@751@752@753@754@755@756@757@758@760@761@762@763@764@765@766@767@768@770@771@772@773@774@775@776@777@778@780@781@782@783@784@785@786@787@788@800@801@802@803@804@805@806@807@808@810@811@812@813@814@815@816@817@818@820@821@822@823@824@825@826@827@828@830@831@832@833@834@835@836@837@838@840@841@842@843@844@845@846@847@848@850@851@852@853@854@855@856@857@858@860@861@862@863@864@865@866@867@868@870@871@872@873@874@875@876@877@878@880@881@882@883@884@885@886@887@888@444@344@544@434@454@443@445

[s3-s1-a]:
https://tjjfvi.github.io/cubular/#012345678123456780234567801345678012456780123567801234678012345780123456801234567123456780234567801345678012456780123567801234678012345780123456801234567012345678234567801345678012456780123567801234678012345780123456801234567012345678123456780345678012456780123567801234678432345780323456801456567012345678123456780234567801456780123567801234678012345780503456801212567012343678123456780234567801345678012567801234678012345780123456801434567012541678123212780234567801345678012456780123678012345780123456801234567012345678123456780234567801345678012456780123567801234780123456801234567012345678123456780234567801345678012456780123567801234678012345801234567012345678123456780234567801345678012456780123567801234678012345780123456-444X1t@@453-445Z1t@@455-344X2t@@345-@@343
