from typing import Hashable


class ConceptNode:
    def __init__(self, name: Hashable, layer: str, layer_ind: int, data: dict | None = None) -> None:
        """Stores multilayer node attributes in a convenient format.

        Parameters
        ----------
        name : Hashable
            A networkx node name. Assumed to be present in every layer.
        layer : str
            A layer ID string for the layer containing this node.
        layer_ind : int
            Index of the layer for the node.
        data : dict | None, optional
            Additional node attributes, in dictionary format. Most algorithms assume a 'weight' field. 
            By default None
        """
        self.name = name
        self.layer = layer
        self.layer_ind = layer_ind
        self.data = data
        self.id = (self.name, self.layer, self.layer_ind)

    def __str__(self) -> str:
        return str((self.name, self.layer, self.layer_ind, self.data))

    def __repr__(self) -> str:
        return 'ConceptNode'+str((self.name, self.layer, self.layer_ind, self.data))