# 🏰 Ferrous Depths

> A terminal roguelike dungeon crawler built in Rust — an iterative algorithm learning project.

```
########################################
#....................................#.#
#.@..................................#.#
#....................................#.#
#..########..........########........#.#
#..#......#..........#......#........#.#
#..#..g...#..........#...>..#........#.#
#..#......#..........#......#........#.#
#..########..........########........#.#
#....................................#.#
########################################

  HP: ████████░░  Level: 3  Floor: 2
```

---

## What This Is

Ferrous Depths is a roguelike dungeon crawler written from scratch in Rust. The game exists for one primary reason: **to build deep intuition for algorithms and data structures through a real, working system.**

Every feature in this project is a canonical computer science algorithm. Not an exercise. Not a toy example. A system where the algorithm is the feature — where getting it wrong means enemies walk through walls, dungeons are disconnected, or the player can see through solid rock.

This is the difference between studying Dijkstra's algorithm and debugging why your enemy is taking a longer path than it should at 11pm.

---

## The Algorithm Map

Each milestone of the project introduces new algorithms. They compound — you cannot implement A\* without understanding Dijkstra, and you cannot implement MCTS without having built Minimax first.

| Milestone | Feature | Core Algorithm | Rust Concept |
|-----------|---------|---------------|--------------|
| 1 | Grid world, movement, game loop | Flat `Vec` index arithmetic | Structs, enums, `match` |
| 2 | Procedural dungeon generation | BSP tree (DFS), flood fill (BFS) | Recursive types with `Box` |
| 3 | Field of view, fog of war | Shadowcasting (DFS), ray casting (BFS) | Traits, `HashSet` |
| 4 | Enemy pathfinding | Dijkstra's map, A\* | `BinaryHeap`, custom `Ord` |
| 5 | Combat, items, ECS architecture | Sparse sets, priority queue turns | `HashMap`, `HashSet` intersections |
| 6 | Intelligent AI, boss enemy | Finite state machine, Minimax, MCTS | Enums as state machines |

---

## Milestone Details

### Milestone 1 — The Engine Core

Build the foundation: a grid world, a player who moves, walls that block, a game loop that runs.

**What you learn:**

The map is a flat `Vec<Tile>` with index arithmetic converting between 2D coordinates and 1D indices. This pattern — `index = y * width + x` — appears in every grid problem in competitive programming and technical interviews. You will write it so many times it becomes automatic.

```rust
fn idx(&self, x: i32, y: i32) -> usize {
    (y * self.width + x) as usize
}
```

The game loop introduces the ownership challenges Rust is known for: systems need mutable access to game state, multiple systems run in sequence, and the borrow checker enforces that no two systems hold conflicting references simultaneously.

**Deliverable:** Player walks around an empty room. Walls block movement. `@` symbol renders in the terminal.

---

### Milestone 2 — Procedural Dungeon Generation

Generate a new dungeon every run. No two games are the same.

**BSP Tree generation (DFS):**

Recursively split the map into two halves along a random axis. Keep splitting until regions are small enough to contain a room. Place a room in each leaf. Connect sibling rooms with corridors. The generation is a DFS traversal of the BSP tree — carving rooms on the way down, connecting corridors on the way up.

```
Map space
├── Left half
│   ├── Top-left quadrant → Room A
│   └── Bottom-left quadrant → Room B
│       Connected by: vertical corridor
└── Right half
    ├── Top-right quadrant → Room C
    └── Bottom-right quadrant → Room D
        Connected by: vertical corridor
Left and Right halves connected by: horizontal corridor
```

This introduces recursive types — `BSPNode` contains child `BSPNode` values. Rust cannot size a type that contains itself, so children are wrapped in `Box<BSPNode>`, placing them on the heap.

**Flood fill connectivity check (BFS):**

After generation, verify the dungeon is fully connected. BFS from the starting room — if any floor tile is unreachable, the dungeon is broken and we regenerate. This is your first real BFS in a practical context where a wrong implementation means the player gets stuck.

**Cellular automata (cave generation):**

An alternative generator: seed the map with random walls, then apply a smoothing rule repeatedly. Five iterations produce organic cave systems. Simple to implement, excellent for understanding grid neighbourhood iteration.

**Deliverable:** Random dungeon every run. Multiple rooms connected by corridors. Stairs to the next floor.

---

### Milestone 3 — Field of View

The player only sees tiles within line of sight. Unexplored tiles are black. Previously seen tiles are dim. Currently visible tiles are full colour.

**Naive ray casting (BFS):**

For each tile within range, trace a line from player to tile. If any wall crosses the line, the tile is not visible. Correct but O(range²) per turn. Build this first — correctness before optimisation.

**Symmetric shadowcasting (recursive DFS):**

Divide the visible area into 8 octants. Cast shadows outward from the player. A wall tile blocks all tiles in its shadow. Recursive structure — each shadow segment spawns child segments around blocking tiles. O(visible_tiles) per turn. The performance difference becomes visible on large maps with many enemies.

This is where implementing a `FieldOfView` trait earns its value — you implement it twice (naive then optimised) and swap between them without touching any other game code.

**Deliverable:** Fog of war. Enemies visible only when in line of sight. The dungeon reveals itself as you explore.

---

### Milestone 4 — Enemy Pathfinding

Monsters chase the player. They navigate around walls. They do not walk through each other.

**Dijkstra map:**

Build a distance map from the player's position to every reachable tile on the map. Each monster consults this map and moves toward lower values — toward the player. The key insight: compute this once per turn for the entire map using BFS from the player, then every enemy gets O(1) pathfinding.

```rust
struct DijkstraMap {
    distances: Vec<f32>,
}

// Enemies just look up their current tile and step to the
// adjacent tile with the lowest distance value.
// No per-enemy path computation needed.
```

This is more powerful than single-target A\* for many enemies — and it introduces an important optimisation pattern: precompute shared structure once rather than repeating the work per entity.

**A\* (specific path planning):**

For cases requiring an explicit path — a boss enemy that plans several moves ahead, an enemy navigating to a position that isn't the player — A\* gives the optimal route. Implemented using `BinaryHeap` from the standard library with a custom ordering to turn Rust's max-heap into a min-heap.

```rust
// BinaryHeap is max-heap by default.
// Implement Ord in reverse to get min-heap behaviour.
// This pattern appears in virtually every priority queue
// interview question.
impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // reversed
    }
}
```

**Deliverable:** Enemies that navigate intelligently around obstacles. Multiple enemies do not stack. The player can be cornered.

---

### Milestone 5 — Combat and the Entity Component System

Turn-based combat. Stats, items, death. A proper game.

**The architectural problem:**

Players, enemies, and items share properties — position, renderable sprite, health, name. In other languages you'd reach for inheritance. Rust has no inheritance. This forces the correct pattern.

**Entity Component System (ECS):**

Entities are plain integer IDs. Components are plain data structs with no behaviour. Systems are functions that operate on entities possessing specific component combinations.

```rust
type Entity = u32;

// Components — pure data
struct Position { x: i32, y: i32 }
struct Health { current: i32, max: i32 }
struct Renderable { glyph: char }
struct Monster;   // marker component — zero size
struct Player;    // marker component — zero size

// World — a collection of sparse component storages
struct World {
    positions:   HashMap<Entity, Position>,
    health:      HashMap<Entity, Health>,
    renderables: HashMap<Entity, Renderable>,
    monsters:    HashSet<Entity>,
    players:     HashSet<Entity>,
}
```

Querying entities with multiple components is a `HashSet` intersection — find all entities that have both a `Position` and a `Health` component. This relational model underpins real game engines (Unity's DOTS, Bevy) and is closely related to how columnar databases store and query data.

**Turn ordering:**

Each entity has an energy counter. Entities gain energy each tick. When energy exceeds a threshold, the entity acts and energy resets. Fast entities act more frequently. Ordering is maintained with a `BinaryHeap` — the same structure built in Milestone 4.

**Deliverable:** Full combat loop. Player fights enemies. Items drop. Game over state. The game is now completable.

---

### Milestone 6 — Intelligence

Smarter enemies. Distinct behaviours. The algorithmic foundations of game AI.

**Finite state machine:**

Enemies transition between states — `Idle`, `Alerted`, `Chasing`, `Attacking`, `Fleeing` — based on conditions like player visibility and health percentage. Implemented as a Rust enum with a `transition()` method. The type system enforces exhaustive handling of every state.

```rust
#[derive(Clone, Copy)]
enum AIState {
    Idle,
    Alerted { last_known: Position },
    Chasing,
    Attacking,
    Fleeing { target: Position },
}
```

**Minimax (boss enemy):**

The boss enemy thinks ahead. Given the current game state, it evaluates all possible move sequences to depth 3 and selects the move that minimises the player's expected outcome. Alpha-beta pruning eliminates branches that cannot affect the result, making the search practical.

This requires a game state that is cheap to copy and fast to evaluate — which is where the bitboard representation from the dependency graph discussion maps directly to game state encoding.

**Monte Carlo rollouts:**

For the most capable enemy, replace the minimax heuristic with Monte Carlo simulation: play out the next N turns randomly M times, average the results, pick the move with the best average outcome. This is the foundation of MCTS — the algorithm behind AlphaGo and modern game AI.

**Deliverable:** Enemies with distinct personalities. The boss requires genuine strategy. The game has replayability.

---

## Project Structure

```
ferrous-depths/
├── src/
│   ├── main.rs              # Entry point, game loop
│   ├── map/
│   │   ├── mod.rs           # Map struct, tile types
│   │   ├── bsp.rs           # BSP tree dungeon generator
│   │   ├── cellular.rs      # Cellular automata generator
│   │   └── dijkstra.rs      # Dijkstra map for pathfinding
│   ├── systems/
│   │   ├── fov.rs           # Field of view (shadowcasting)
│   │   ├── pathfinding.rs   # A* implementation
│   │   ├── combat.rs        # Combat resolution
│   │   └── ai.rs            # Enemy AI (FSM, Minimax, MCTS)
│   ├── components/          # ECS component definitions
│   ├── world.rs             # ECS world and queries
│   └── render.rs            # Terminal rendering
├── Cargo.toml
└── README.md
```

---

## Getting Started

```bash
git clone https://github.com/yourusername/ferrous-depths
cd ferrous-depths
cargo run
```

**Dependencies:**

```toml
[dependencies]
bracket-lib = "0.8"   # Terminal rendering and geometry
rand        = "0.8"   # Procedural generation
```

**Controls:**

```
Arrow keys / hjkl   Move
.                   Wait (skip turn)
g                   Pick up item
i                   Open inventory
q                   Quit
```

---

## Algorithm Reference

Each algorithm implemented in this project maps to a class of interview problems.

**BFS — Breadth-First Search**
Used in: flood fill connectivity, ray casting field of view, Dijkstra map construction.
Interview problems: shortest path in unweighted graph, island counting, word ladder.

**DFS — Depth-First Search**
Used in: BSP tree traversal, shadowcasting field of view, maze generation.
Interview problems: cycle detection, topological sort (with three-colour marking), connected components.

**Dijkstra's Algorithm**
Used in: enemy pathfinding distance map.
Interview problems: shortest path in weighted graph, network delay time.

**A\***
Used in: boss enemy specific path planning.
Interview problems: shortest path with heuristic, grid navigation with obstacles.

**Binary Heap / Priority Queue**
Used in: A\* open set, turn order system.
Interview problems: K largest elements, merge K sorted lists, task scheduling.

**Minimax with Alpha-Beta Pruning**
Used in: boss enemy move selection.
Interview problems: game tree evaluation, optimal play problems.

**Finite State Machine**
Used in: enemy behaviour system.
Interview problems: string parsing, regex matching, protocol state machines.

**Sparse Set / ECS (HashMap-based)**
Used in: entity component system.
Interview problems: group anagrams, two-sum variants, relational queries.

---

## Why Rust

Rust is not the easiest language to write a game in. It is the most educational.

Every ownership error the compiler surfaces is a real bug that would be a subtle data race or use-after-free in C++. Every lifetime annotation is a contract about how long data lives. The borrow checker forces you to think about state ownership before you write it.

The algorithms here — particularly the ECS, the pathfinding, and the AI systems — are the kinds of systems where Rust's model produces genuinely cleaner architecture than an object-oriented approach. You cannot accidentally share mutable state between systems. The type system enforces component query correctness. State machine transitions are exhaustively checked by the compiler.

By Milestone 4 you will have written enough Rust to have real opinions about it. That is the goal.

---

## Roadmap

- [ ] Milestone 1 — Engine core
- [ ] Milestone 2 — Procedural generation
- [ ] Milestone 3 — Field of view
- [ ] Milestone 4 — Pathfinding
- [ ] Milestone 5 — Combat and ECS
- [ ] Milestone 6 — AI systems
- [ ] Extended — Multiplayer via WebSockets
- [ ] Extended — Web assembly build (play in browser)
- [ ] Extended — Persistent leaderboard

---

## References

- *Roguelike Tutorial in Rust* — Herbert Wolverson (bracket-lib creator)
- *Game Programming Patterns* — Robert Nystrom
- *Red Blob Games* — Amit Patel (exceptional visualisations of every algorithm used here)
- *Procedural Content Generation in Games* — Shaker, Togelius, Nelson

---

*Built as a deliberate algorithm learning project. Every system is an excuse to implement something that would otherwise be an abstract exercise.*