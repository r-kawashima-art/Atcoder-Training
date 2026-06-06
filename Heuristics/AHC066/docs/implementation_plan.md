# AHC066 Solution V2 Implementation Plan

## Problem Analysis & Strategy
Our baseline solution (V1) achieved a score of 594 by using a greedy nearest-neighbor approach. To significantly improve this score, we must optimize the sequence in which the robot picks up and delivers the balls, and then compress the resulting operation sequence using macros.

Since N $\le$ 20 and M $\le$ 40, the grid is small. 
1. **Precomputation (APSP)**: We can precompute the All-Pairs Shortest Path (APSP) between all possible directed states `(row, col, direction)` using BFS. There are at most $20 \times 20 \times 4 = 1600$ states, making APSP very fast to compute ($1600 \times 1600 \approx 2.5 \times 10^6$ iterations).
2. **TSP Formulation**: The task of delivering $M$ balls is equivalent to finding an optimal permutation of the balls. For a given permutation $P = (p_1, p_2, \dots, p_M)$, the robot's sequence of locations is:
   $(0,0) \rightarrow \text{Ball } p_1 \rightarrow \text{Basket } p_1 \rightarrow \text{Ball } p_2 \rightarrow \text{Basket } p_2 \rightarrow \dots$
   Because initial ball and basket positions are distinct, and we only drop a ball at its correct basket, the robot will always become empty-handed after dropping a ball. There will be no unintended swaps.
3. **Dynamic Programming**: For a fixed permutation, the cost depends on the direction the robot is facing at each step. We can use DP to evaluate the optimal sequence of directions for a permutation in $O(M)$ time.
4. **Simulated Annealing (SA)**: We will use SA to explore the space of permutations. The state is the permutation, the neighborhood involves swapping two elements or reversing a subsegment (2-opt), and the energy is the DP cost. We will run this for about 1.8 seconds.
5. **Macro Compression**: After obtaining the optimal `F, R, L, S` sequence, we will find the most profitable substring to record as a macro (`M ... M`) and replace its subsequent non-overlapping occurrences with `P`, further reducing the final operation count.

## Proposed Changes

### [Component: AHC066 Solution v2]
Create `AHC066_solution_v2.rs` containing:
- BFS for APSP between all `(r, c, dir)` states.
- DP function `evaluate_permutation(&perm, &apsp, &data) -> (usize, Vec<usize>)` to calculate the shortest path cost and the optimal directions at each waypoint.
- SA implementation with time-based exponential cooling.
- Macro compression function `compress_sequence(&ops) -> Vec<char>`.
- Main execution loop and output formatting.

#### [NEW] [AHC066_solution_v2.rs](file:///c:/Users/kawas/OneDrive/Atcoder/working/Heuristics/AHC066/src/AHC066_solution_v2.rs)
- Implement state graph building.
- Implement APSP.
- Implement SA loop.
- Implement Macro compression.

## Verification Plan
### Automated Tests
- Build `AHC066_solution_v2.rs` in the `playground` project.
- Run against `toolsAHC066/in/0000.txt`.
- Compare the new score (using `vis.exe`) against the V1 score of 594. We expect a significantly lower score.

## Open Questions
> [!IMPORTANT]
> The time limit is 2.0 seconds. Simulated Annealing needs a time limit to know when to stop. I plan to set the time limit to 1.8 seconds to leave a margin for I/O, APSP precomputation, and macro compression. Is this acceptable?
