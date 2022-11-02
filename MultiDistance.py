from __future__ import annotations
from copy import deepcopy
import numpy as np


class MultiDistance():
    @classmethod
    def from_array(cls, arr:np.ndarray[np.number]):
        x = cls(len(arr))
        x.arr = np.array([x for x in arr])
        return x
    
    def __init__(self, nlayers: int) -> None:
        """Initializes MultiDistance from dictionary.

        Parameters
        ----------
        nlayers : int
        """
        self.arr = np.zeros(nlayers)

    def add_distance_to_layer(self, distance: float, layer: int):
        """Increases the multidistance entry by distance in the specified layer

        Parameters
        ----------
        distance : float
            Amount to increase distance by.
        layer : int
            Index of layer to increase distance in.
        """
        self.arr[layer] += distance

    def add(self, other: MultiDistance) -> None:
        """Computes element-wise addition of a distance tuple (in place).

        Parameters
        ----------
        other : MultiDistance
            Other MultiDistance to add to this one.
        """
        self.arr += other.arr

    def __add__(self, other: MultiDistance) -> MultiDistance:
        """Computes element-wise addition of distance tuples.

        Parameters
        ----------
        other : MultiDistance
            Other MultiDistance to add to this one.

        Returns
        -------
        MultiDistance
            Element-wise sum of this distance tuple and other.
        """
        res = deepcopy(self)
        res.add(other)
        return res

    # domination relation, assume positive weights
    def __lt__(self, other: MultiDistance) -> bool:
        """Computes the domination relation, which is the standard partial order on R^n.

        Parameters
        ----------
        other : MultiDistance
            The MultiDistance for comparison

        Returns
        -------
        bool
            True if this MultiDistance is dominated by the other.
        """
        diff = other.arr - self.arr
        return all(diff >= 0) and not all(diff == 0)

    def __gt__(self, other: MultiDistance) -> bool:
        return other.__lt__(self)

    def __le__(self, other: MultiDistance) -> bool:  # dominate or incomparable
        diff = other.arr - self.arr
        return all(diff >= 0)

    def __ge__(self, other: MultiDistance) -> bool:
        return other.__le__(self)

    def __eq__(self, other: MultiDistance) -> bool:
        return all(self.arr == other.arr)

    def is_equivalent_to(self, other: MultiDistance) -> bool:
        return not(self < other) and not(other < self)

    def __str__(self) -> str:
        return self.arr.__str__()
    
    def __repr__(self) -> str:
        return self.arr.__str__()
    
    def __hash__(self) -> int:
        return hash(self.arr)

    def compare(self, other: MultiDistance) -> int:
        """Compare this MultiDistance to other using the domination relation, which is the standard
        partial order on R^n.

        Parameters
        ----------
        other : MultiDistance
            The MultiDistance to compare to.

        Returns
        -------
        int
            If this MultiDistance is dominated by the other, return -1; if this MultiDistance dominates
            the other, return 1; otherwise, return 0.
        """
        diff = other.arr - self.arr
        if all(diff == 0):
            return 0
        elif all(diff >= 0):
            return -1
        elif all(diff <= 0):
            return 1
        else:
            return 0


def multimin(dists: list[MultiDistance]) -> list[MultiDistance]:
    """Find elements of the input list that is not dominated by any other member of the list.

    Parameters
    ----------
    dists : list[MultiDistance]
        A list of MultiDistances to minimize.

    Returns
    -------
    list[MultiDistance]
        Elements of the input list that are not dominated by any other member of the list.
    """
    final = []
    for i, t in enumerate(dists):
        for c in dists[i+1:]+final:
            if c < t or c == t:  # Note: not the same as t>=c because only have partial order
                break
        else:  # never hit the "break"
            final.append(t)
    return final


# Like multimin(d1+d2, but assumes d1 and d2 are already internally incomparable and reduced)
def multimerge(d1: list[MultiDistance], d2: list[MultiDistance]) -> list[MultiDistance]:
    """Computes multimin(d1+d2), but assumes that d1 and d2 are already internally incomparable 
    and reduced. This assumption is NOT checked.

    Parameters
    ----------
    d1 : list[MultiDistance]
        First list to merge.
    d2 : list[MultiDistance]
        Second list to merge.

    Returns
    -------
    list[MultiDistance]
        Elements of the input lists that are not dominated by any member of either input list.
    """
    final = []
    l2 = [1]*len(d2)
    for c1 in d1:
        for j, c2 in enumerate(d2):
            s = c1.compare(c2)
            l2[j] += s
            if s > 0 or c1 == c2:
                break
        else:
            final.append(c1)
    for s, c in zip(l2, d2):
        if s > 0:
            final.append(c)
    return final


def sum_pareto_distance(dist: MultiDistance,
                        layer_weights: np.ndarray[float] | None = None) -> float:
    """Computes a weighted sum of a MultiDistance.

    Parameters
    ----------
    dist : MultiDistance
        MultiDistance to compute the weighted sum of.
    layer_weights : np.ndarray[float] | None, optional
        How much to weight each layer pair. If None (default), then layer pairs are weighted by 1.

    Returns
    -------
    float
        _description_
    """

    if layer_weights is None:
        return dist.arr.sum()
    else:
        return np.dot(layer_weights, dist.arr)
