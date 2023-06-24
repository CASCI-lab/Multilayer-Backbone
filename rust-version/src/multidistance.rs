use std::collections::HashMap;
use std::ops::Add;

#[must_use]
pub fn multimin(dists: &[MultiDistance]) -> Vec<MultiDistance> {
    let mut minlist = Vec::new();

    for (i, t) in dists.iter().enumerate() {
        let mut found_smaller = false;
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

#[derive(PartialEq, Clone, Debug, Default)]
pub struct MultiDistance {
    pub total: HashMap<EdgeLayerID, f32>,
}

impl MultiDistance {
    pub fn add_to_self(&mut self, rhs: &Self) {
        for (key, value) in &rhs.total {
            *self.total.entry(*key).or_insert(0.0) += value;
        }
    }
}

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
        for (key, rhs) in &other.total {
            let lhs = *self.total.get(key).unwrap_or(&0.0);
            if lhs < *rhs {
                found_larger = true;
            } else if lhs > *rhs {
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

        let dists = vec![m1.clone(), m2.clone(), m3.clone(), m4.clone()];
        let dists2 = vec![m1.clone(), m2.clone(), m3.clone()];
        let dists3 = vec![m1.clone(), m1.clone(), m2.clone()];
        let mm = multimin(&dists);
        let mm2 = multimin(&dists2);
        let mm3 = multimin(&dists3);
        assert_eq!(multimin(&Vec::new()), Vec::new());
        assert_eq!(mm, vec![m4.clone()]);
        assert_eq!(mm2, vec![m1.clone(), m2.clone()]);
        assert_eq!(mm3, vec![m1.clone(), m2.clone()]);

        assert!(m1.partial_cmp(&m2).is_none());
        assert!(m1 < m3);
        assert!(m1 > m4);
        assert!(m2 < m3);
        assert!(m2 > m4);
        assert!(m4 < m3);

        assert_eq!(m1 + m2, m3 + m4);
    }
}
