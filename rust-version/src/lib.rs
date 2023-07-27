mod closure;
mod conversion;
mod multidistance;
mod shortest_paths;

pub use closure::*;
pub use conversion::*;
pub use multidistance::*;
pub use shortest_paths::*;

#[must_use]
pub fn distance_closure(edges: &[(usize, usize, usize, usize, f32)]) -> MultidistanceClosure {
    let multiplex = edges_to_multiplex(edges);
    multidistance_closure(&multiplex)
}
