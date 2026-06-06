# Atcoder Heuristic Contest 066

## Story

CEO Takahashi of AtCoder Inc. likes ball games. However, while playing with various balls during breaks from work, he scattered balls all over the office.

Seeing this, Vice President Aoki became very angry and ordered Takahashi to clean them up immediately. At a loss, Takahashi decided to use the cleaning robot he had recently purchased to put away the balls.

This robot can move around the office and pick up or put down balls using the basic operations of moving forward, turning right, turning left, and swapping. Furthermore, the included controller has a macro function, which can record a sequence of operations and play it back later.

Instead of Takahashi, put all the balls into their corresponding baskets using as short a sequence of operations as possible.

## Problem Statement

There is an N×N grid. Let the coordinates of the top-left cell be (0,0), and let the coordinates of the cell i cells downward and j cells to the right be (i,j).
The outer boundary of the grid is surrounded by walls, and there may also be walls between adjacent cells. It is guaranteed that every cell is reachable from every other cell by repeatedly moving up, down, left, and right without crossing walls.
On the grid, there are M types of balls and M types of baskets. For each type k (0≤k<`M`), there is one ball of type k and one basket of type k. In the initial state, each cell contains at most one ball or basket.
The robot is initially at cell (0,0) and facing right. It is not holding any ball initially.
You can perform the following four types of basic operations on the robot.

- Move forward F: Move one cell in the direction the robot is currently facing. If there is a wall between the current cell and the destination cell, the robot does not move and remains in place.
- Turn right R: Turn 90 degrees clockwise without moving from the current position.
- Turn left L: Turn 90 degrees counterclockwise without moving from the current position.
- Swap S: Swap the ball currently held by the robot with the ball placed at the current position. If the robot is not holding a ball and there is a ball at the current position, it picks up that ball, and the current position becomes empty of balls. If the robot is holding a ball and there is no ball at the current position, it places that ball at the current position, and the robot becomes empty-handed. If the robot is holding a ball and there is also a ball at the current position, the robot swaps the ball it is holding with the ball at the current position. If the robot is not holding a ball and there is no ball at the current position, nothing happens. A ball placed on a cell with a basket is also swapped by the swap operation.
Takahashi's controller also has the following two types of operations.
- Macro M: If a macro is not currently being recorded, start recording a macro. If a macro is currently being recorded, stop recording and register the recorded sequence of operations as a new macro.
- Play P: Play back the most recently registered macro. If no macro has been registered, nothing happens.
If a basic operation F, R, L, or S is performed while recording a macro, that operation is executed and appended to the end of the macro being recorded.
If P is performed while recording a macro, the most recently registered macro is played back, not the macro currently being recorded. At this time, the sequence of basic operations that is actually played back is appended to the end of the macro being recorded.
For example, consider the case where the registered macro is RFF and the operation sequence MFPM is executed. First, M starts recording a macro, and then F is executed and recorded. Next, P plays back the registered macro RFF, which is also appended to the macro being recorded. Finally, M stops recording. The sequence of basic operations executed during this period is FRFF, and the newly registered macro is also FRFF.
Initially, no macro is registered. Also, no macro is being recorded initially.
The upper limit T on the number of basic operations is given as input. After macro expansion, at most T basic operations are executed; the (T+1)-st basic operation is not executed, and execution is terminated. Aim to put all balls into their corresponding baskets using as short an operation sequence as possible.

## Scoring

Let A be the length of the output operation sequence. Here, M and P are each counted as one button operation.
Let V be the number of balls placed on the cells of their corresponding baskets at the end of the simulation.
For each test case, the following absolute score is obtained.
If V=`M`, `A`. If V<`M`, `T`×(`M`−`V`).
The lower the absolute score, the better.

## Input

Input is given from Standard Input in the following format:

```text
N
M
T
v0
⋮
vN-1
h0
⋮
hN-2
b0 c0 d0 e0
⋮
bM-1 cM-1 dM-1 eM-1
```

Here, \((b_k, c_k)\) represents the initial position of the ball of type \(k\), and \((d_k, e_k)\) represents the position of the basket of type \(k\).

### Constraints

- \(10 \leq N \leq 20\)
- \(\frac{N}{2} \leq M \leq 2N\)
- \(1 \leq T \leq 2N^2M\)

### Vertical Walls

- \(v_i = v_{i,0} \cdots v_{i,N-2}\) is a string of length \(N-1\) consisting of `0` and `1`.
- Its \(j\)-th character \(v_{i,j}\) indicates whether there is:
  - `1`: a wall between cells \((i,j)\) and \((i,j+1)\)
  - `0`: no wall between those cells

### Horizontal Walls

- \(h_i = h_{i,0} \cdots h_{i,N-1}\) is a string of length \(N\) consisting of `0` and `1`.
- Its \(j\)-th character \(h_{i,j}\) indicates whether there is:
  - `1`: a wall between cells \((i,j)\) and \((i+1,j)\)
  - `0`: no wall between those cells

### Guarantees

- Every cell is reachable from every other cell.
- All initial positions of balls and positions of baskets are distinct.

## Output

Let \(A\) be the length of the operation sequence, and let \(a_t\) be the character (`F`, `R`, `L`, `S`, `M`, `P`) representing the \(t\)-th operation.

Output to Standard Output in the following format:

```text
a_0
a_1
⋮
a_{A-1}
```

### Constraint

- The length \(A\) of the output operation sequence must be at most \(T\).

## Time and Space Constraint

Time Limit: 2 sec / Memory Limit: 1024 MiB

## Tools for AI

- **Execution Directory**: `C:\Users\kawas\OneDrive\Atcoder\working\Heuristics\AHC066\toolsAHC066`
- **Playground**: `C:\Users\kawas\OneDrive\Atcoder\playground`
- **Output Directory**: `C:\Users\kawas\OneDrive\Atcoder\working\Heuristics\AHC066\src\`
