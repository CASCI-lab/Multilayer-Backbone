use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};
use std::ops::Add;
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeID(pub usize);

impl IntoPy<PyObject> for NodeID {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let NodeID(node_id_val) = self;
        // PyObject::from(PyInt::new(py, node_id_val))
        node_id_val.into_py(py)
    }
}

#[must_use]
pub fn multimin(dists: &[MultiDistance]) -> Vec<MultiDistance> {
    let mut minlist = Vec::new();
    let mut found_smaller;
    for (i, t) in dists.iter().enumerate() {
        found_smaller = false;
        for c in dists[(i + 1)..dists.len()].iter().chain(minlist.iter()) {
            if c <= t {
                found_smaller = true;
                break;
            }
        }
        if !found_smaller {
            minlist.push(t.clone());
        }
    }
    minlist
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct EdgeLayerID {
    pub layer_start: usize,
    pub layer_end: usize,
    pub layer_weight_index: usize,
}

impl IntoPy<PyObject> for EdgeLayerID {
    fn into_py(self, py: Python<'_>) -> PyObject {
        ((self.layer_start, self.layer_end), self.layer_weight_index).into_py(py)
    }
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct MultiDistance {
    pub total: HashMap<EdgeLayerID, f32>,
}

impl MultiDistance {
    pub fn add_to_self(&mut self, rhs: &Self) {
        for (key, value) in &rhs.total {
            #[allow(clippy::float_cmp)] // this is ok because we only care about literal zero
            if value != &0.0 {
                *self.total.entry(*key).or_insert(0.0) += value;
            }
        }
    }
}

impl IntoPy<PyObject> for MultiDistance {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.total.into_py(py)
    }
}

impl Eq for MultiDistance {}

impl Add for MultiDistance {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;

        result.add_to_self(&rhs);
        result
    }
}

impl PartialOrd for MultiDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut found_larger = false;
        let mut found_smaller = false;

        let keys = other
            .total
            .iter()
            .chain(self.total.iter())
            .map(|(k, _)| k)
            .collect::<HashSet<&EdgeLayerID>>();

        for key in keys {
            let lhs = *self.total.get(key).unwrap_or(&0.0);
            let rhs = *other.total.get(key).unwrap_or(&0.0);
            if lhs < rhs {
                found_larger = true;
            } else if lhs > rhs {
                found_smaller = true;
            }

            if found_larger && found_smaller {
                return None;
            }
        }

        match (found_larger, found_smaller) {
            (false, false) => Some(std::cmp::Ordering::Equal),
            (false, true) => Some(std::cmp::Ordering::Greater),
            (true, false) => Some(std::cmp::Ordering::Less),
            (true, true) => None, // never reached because we return early from loop
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_partial_order() {
        let layer1 = EdgeLayerID {
            layer_start: 0,
            layer_end: 0,
            layer_weight_index: 0,
        };
        let layer2 = EdgeLayerID {
            layer_start: 0,
            layer_end: 1,
            layer_weight_index: 0,
        };

        let m1 = MultiDistance {
            total: HashMap::from([(layer1, 1.0), (layer2, 2.0)]),
        };

        let m2 = MultiDistance {
            total: HashMap::from([(layer1, 2.0), (layer2, 1.0)]),
        };

        let m3 = MultiDistance {
            total: HashMap::from([(layer1, 2.0), (layer2, 2.0)]),
        };

        let m4 = MultiDistance {
            total: HashMap::from([(layer1, 1.0), (layer2, 1.0)]),
        };

        let m5 = MultiDistance {
            total: HashMap::from([(layer2, 3.0)]),
        };

        // testing basic equalities
        assert!(m5.partial_cmp(&m1).is_none());
        assert!(m5.partial_cmp(&m2).is_none());
        assert!(m5.partial_cmp(&m3).is_none());
        assert!(m5.partial_cmp(&m4).is_none());
        assert!(m1.partial_cmp(&m5).is_none());
        assert!(m2.partial_cmp(&m5).is_none());
        assert!(m3.partial_cmp(&m5).is_none());
        assert!(m4.partial_cmp(&m5).is_none());
        assert!(m1.partial_cmp(&m2).is_none());
        assert!(m2.partial_cmp(&m1).is_none());
        assert!(m1 < m3);
        assert!(m1 > m4);
        assert!(m2 < m3);
        assert!(m2 > m4);
        assert!(m4 < m3);
        assert!(m3 > m1);
        assert!(m4 < m1);
        assert!(m3 > m2);
        assert!(m4 < m2);
        assert!(m3 > m4);

        let dists = vec![m1.clone(), m2.clone(), m3.clone(), m4.clone()];
        let dists2 = vec![m1.clone(), m2.clone(), m3.clone()];
        let dists3 = vec![m1.clone(), m1.clone(), m2.clone()];
        let mm = multimin(&dists);
        let mm2 = multimin(&dists2);
        let mm3 = multimin(&dists3);

        // testing multimin stuff
        assert_eq!(multimin(&Vec::new()), Vec::new());
        assert_eq!(&mm, &vec![m4.clone()]);
        assert_eq!(&mm2, &vec![m1.clone(), m2.clone()]);
        assert_eq!(&mm3, &vec![m1.clone(), m2.clone()]);
        assert_eq!(
            &vec![m5.clone(), m1.clone()],
            &multimin(&[m5.clone(), m1.clone()])
        );
        assert_eq!(&vec![m1.clone(), m5.clone()], &multimin(&[m1.clone(), m5]));

        // testing addition
        assert_eq!(m1 + m2, m3 + m4);
    }
}
