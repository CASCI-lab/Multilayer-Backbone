from typing import Hashable


class ConceptNode:
    def __init__(self, name: Hashable, layer: str, data: dict | None = None) -> None:
        """Stores multilayer node attributes in a convenient format.

        Parameters
        ----------
        name : Hashable
            A networkx node name. Assumed to be present in every layer.
        layer : str
            A layer ID string for the layer containing this node.
        data : dict | None, optional
            Additional node attributes, in dictionary format. Most algorithms assume a 'weight' field. 
            By default None
        """
        self.name = name
        self.layer = layer
        self.data = data
        self.id = (self.name, self.layer)

    def __str__(self) -> str:
        return str((self.name, self.layer, self.data))

    def __repr__(self) -> str:
        return 'ConceptNode'+str((self.name, self.layer, self.data))