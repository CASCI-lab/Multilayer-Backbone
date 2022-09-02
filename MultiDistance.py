from __future__ import annotations
from copy import deepcopy
    

class MultiDistance(dict):
    def __init__(self, dists: dict[tuple[str,str], float]) -> None:
        """Initializes MultiDistance from dictionary.

        Parameters
        ----------
        dists : dict[tuple[str,str], float]
            Keys correspond to layer pairs, and values correspond to distance in that layer pair.
            Generally, the two layers in the key pair will be the same.
        """
        super().__init__(dists)

    def add(self, other: MultiDistance) -> None:
        """Computes element-wise addition of a distance tuple (in place).

        Parameters
        ----------
        other : MultiDistance
            Other MultiDistance to add to this one.
        """
        for k, v in other.items():
            self[k] = self.get(k, 0) + v

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
        better_in_one = False
        for k in self:
            if k not in other:
                return False
            if self[k] > other[k]:
                return False
            if self[k] < other[k]:
                better_in_one = True
        return better_in_one

    def __gt__(self, other: MultiDistance) -> bool:
        return other.__lt__(self)

    def __le__(self, other: MultiDistance) -> bool:  # dominate or incomparable
        for k in self:
            if k not in other:
                return False
            if self[k] > other[k]:
                return False
        return True

    def __ge__(self, other: MultiDistance) -> bool:
        return other.__le__(self)

    def is_equivalent_to(self, other: MultiDistance) -> bool:
        return not(self < other) and not(other < self)

    def __str__(self) -> str:
        return super().__str__()

    def __hash__(self) -> int:
        # Should never be nested, so this is ok. Should check though.
        return hash(frozenset(self.items()))

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
        smaller_in_one = False
        larger_in_one = False
        for k in self | other:
            if k not in other:
                larger_in_one = True
            elif k not in self:
                smaller_in_one = True
            else:                
                if self[k] > other[k]:
                    larger_in_one = True
                elif self[k] < other[k]:
                    smaller_in_one = True
            
            if smaller_in_one and larger_in_one:
                return 0
        if smaller_in_one and not larger_in_one:
            return -1
        if not smaller_in_one and larger_in_one:
            return 1
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
        for j,c2 in enumerate(d2):
            s = c1.compare(c2)
            l2[j] += s
            if s > 0 or c1==c2:
                break
        else:
            final.append(c1)
    for s,c in zip(l2,d2):
        if s > 0:
            final.append(c)
    return final

def sum_pareto_distance(dist: MultiDistance,
                        layer_weights: dict[tuple[str, str], float] | None = None) -> float:
    """Computes a weighted sum of a MultiDistance.

    Parameters
    ----------
    dist : MultiDistance
        MultiDistance to compute the weighted sum of.
    layer_weights : dict[tuple[str, str], float] | None, optional
        How much to weight each layer pair. If None (default), then layer pairs are weighted by 1.

    Returns
    -------
    float
        _description_
    """

    if layer_weights is None:
        return sum([v for v in dist.values()])
    else:
        return sum([layer_weights[k]*v for k, v in dist.items()])
