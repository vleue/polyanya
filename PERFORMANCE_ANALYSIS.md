# Performance Issues Analysis for Polyanya

This document identifies potential performance issues in the Polyanya pathfinding library based on code analysis.

## Critical Performance Issues

### 1. Inefficient Loop Bound in Pathfinding Algorithm

**Location**: `src/lib.rs:469`
```rust
for _ in 0..self.layers.iter().map(|l| l.polygons.len()).sum::<usize>() * 10 {
```

**Issue**: The loop bound is recalculated on every pathfinding call, which involves:
- Iterating through all layers
- Summing all polygon counts 
- Multiplying by 10

**Impact**: O(L) complexity for each pathfinding call where L is the number of layers. For large meshes with many layers, this becomes expensive.

**Recommendation**: Cache this value after mesh construction/modification.

### 2. Unnecessary Vector Allocation in Polygon Merging

**Location**: `src/merger.rs:44`
```rust
for [edge0, edge1] in poly.edges_index().collect::<Vec<_>>() {
```

**Issue**: Creating a temporary vector for iteration when the iterator could be used directly.

**Impact**: Unnecessary heap allocation for each polygon during merging.

**Recommendation**: Remove `.collect::<Vec<_>>()` and iterate directly.

### 3. Expensive Vec Operations in Search Results

**Location**: `src/lib.rs:479` and `src/lib.rs:494`
```rust
Some(paths.remove(0))  // Called twice
```

**Issue**: `Vec::remove(0)` has O(n) complexity as it shifts all remaining elements.

**Impact**: Performance degrades with the number of paths found.

**Recommendation**: Use `VecDeque` or `paths.swap_remove(0)` if order doesn't matter.

### 4. Redundant Data Structure Access

**Location**: `src/instance.rs:396`
```rust
.collect::<Vec<_>>();
```

**Issue**: Converting iterator to vector when the iterator could be used directly in many cases.

**Impact**: Unnecessary allocations in hot paths.

## Memory Management Issues

### 5. Clone Operations in Hot Paths

**Location**: `src/instance.rs:559-561`
```rust
let mut path = node.path.clone();
let mut path_with_layers = node.path_with_layers.clone();
```

**Issue**: Cloning potentially large SmallVecs during pathfinding search.

**Impact**: Memory pressure and allocation overhead during search.

**Recommendation**: Consider using reference counting or restructuring to avoid clones.

### 6. HashMap for Root History

**Location**: `src/instance.rs:63`
```rust
pub(crate) root_history: HashMap<Root, f32>,
```

**Issue**: Using HashMap with Vec2 coordinates as keys requires hashing floating-point values.

**Impact**: Hash collisions and precision issues with floating-point keys.

**Recommendation**: Consider using a spatial data structure or integer-based keys.

### 7. Multiple Vec::new() Allocations in Tests

**Location**: Multiple locations in `src/lib.rs`
```rust
path: SmallVec::new(),
path_with_layers: SmallVec::new(),
path_through_polygons: SmallVec::new(),
```

**Issue**: Repeated small allocations for test structures.

**Impact**: Test performance degradation.

**Recommendation**: Use const default values or implement Default trait.

## Algorithmic Inefficiencies

### 8. Area Calculation and Sorting in Merger

**Location**: `src/merger.rs:28-36`
```rust
let mut area = self
    .polygons
    .iter()
    .enumerate()
    .map(|(i, poly)| (i, poly.area(self)))
    .collect::<Vec<_>>();
area.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap().reverse());
```

**Issue**: Computing area for all polygons then sorting, when only a subset might be needed.

**Impact**: O(n log n) sorting overhead for potentially unnecessary work.

**Recommendation**: Consider lazy evaluation or early termination strategies.

### 9. Repeated Distance Calculations

**Location**: Multiple locations in pathfinding code
```rust
vertex.coords + offset).distance_squared(point)
```

**Issue**: Repeated distance calculations in successor generation without caching.

**Impact**: Unnecessary floating-point operations in inner loops.

**Recommendation**: Cache commonly used distance calculations.

## Data Structure Optimizations

### 10. Inefficient Data Layout for Cache Performance

**Issue**: Structures like `SearchNode` may have poor cache locality due to mixed data types and sizes.

**Impact**: Cache misses during intensive pathfinding operations.

**Recommendation**: 
- Reorder struct fields by size (largest to smallest)
- Consider separating hot and cold data
- Use struct-of-arrays for bulk operations

### 11. BinaryHeap Push/Pop Overhead

**Location**: `src/instance.rs:61`
```rust
pub(crate) queue: BinaryHeap<SearchNode>,
```

**Issue**: BinaryHeap operations may be suboptimal for pathfinding workloads.

**Impact**: O(log n) operations for priority queue management.

**Recommendation**: Consider specialized priority queues for pathfinding (e.g., bucket queues for discrete costs).

## Computational Optimizations

### 12. Floating-Point Precision Issues

**Location**: `src/instance.rs:31-36`
```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    ((self.0.x * PRECISION) as i32).hash(state);
    ((self.0.y * PRECISION) as i32).hash(state);
}
```

**Issue**: Converting floating-point to integer for hashing may cause precision loss and hash collisions.

**Impact**: Reduced hash quality and potential performance degradation.

**Recommendation**: Use a more robust floating-point hashing approach or spatial indexing.

### 13. Unnecessary String Operations in Stats

**Location**: `src/lib.rs:437-446`
```rust
eprintln!(
    "index;micros;successor_calls;generated;pushed;popped;pruned_post_pop;length",
);
eprintln!(
    "{};{};0;0;0;0;0;{}",
    // ...
);
```

**Issue**: String formatting in release builds when stats feature is enabled.

**Impact**: Performance overhead for logging in production.

**Recommendation**: Use structured logging or disable in release builds.

## Recommendations Summary

### High Priority
1. Cache the polygon count calculation in pathfinding loop
2. Remove unnecessary `.collect()` calls in hot paths
3. Replace `Vec::remove(0)` with more efficient alternatives
4. Optimize data structure layouts for cache performance

### Medium Priority
5. Reduce clone operations in search algorithms
6. Improve floating-point hashing strategy
7. Consider specialized data structures for specific use cases

### Low Priority
8. Optimize test code allocations
9. Review string formatting overhead
10. Consider algorithmic improvements for polygon merging

These optimizations should be implemented incrementally with benchmarking to validate performance improvements.