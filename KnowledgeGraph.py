from __future__ import annotations
from copy import deepcopy

import numpy as np
from MultiDistance import MultiDistance, sum_pareto_distance, multimerge
from ConceptNode import ConceptNode
from itertools import count, pairwise
import networkx as nx
from heapq import heappop, heappush


from typing import Hashable, Iterator

nodeIDType = tuple[Hashable, str, int]


class KnowledgeGraph:
    def __init__(self, layers: dict[str, nx.DiGraph]) -> None:
        """Initializes a multilayer knowledge graph from a dictionary of layers.

        Parameters
        ----------
        layers : dict[str, nx.DiGraph]
            Each key is a ID for the layer, and each value is a networkx digraph representing the layer.
        """
        self.layers = layers
        self.layer_list = list(sorted(layers))
        self.layer_ind = {L: i for i, L in enumerate(sorted(layers))}
        self.nodes = {(u, layer_id): ConceptNode(u, layer_id, self.layer_ind[layer_id], d)
                      for layer_id, G in layers.items()
                      for u, d in G.nodes(data=True)}
        self.n_layers = len(layers)

    def conceptnode(self, node_id: tuple[Hashable, str]) -> ConceptNode:
        """Constructs a ConceptNode from a node name and a layer ID.

        Parameters
        ----------
        node_id : tuple[Hashable, str]
            The node name and layer ID of the ConceptNode to construct.

        Returns
        -------
        ConceptNode
            Constructed ConceptNode.
        """
        u, layer_id = node_id
        d = self.layers[layer_id].nodes[u]
        return ConceptNode(u, layer_id, self.layer_ind[layer_id], d)

    def neighbors(self, node: ConceptNode) -> set[ConceptNode]:
        """Computes and returns the set of neighbors of the input node.

        Parameters
        ----------
        node : ConceptNode
            ConceptNode whose neighbors we wish to find.

        Returns
        -------
        set[ConceptNode]
            The set of neighbors of the input node.
        """
        vs = set()

        # interlayer
        for layer_id in self.layers.keys():
            if layer_id != node.layer:
                vs.add(self.nodes[(node.name, layer_id)])

        # intralayer
        G = self.layers[node.layer]
        for v in G.neighbors(node.name):
            vs.add(self.nodes[(v, node.layer)])

        return vs

    def edge_weight(self, u: ConceptNode, v: ConceptNode, weight: str = 'weight') -> float:
        """Computes the edge weight between two ConceptNodes in the KnowledgeGraph.

        Parameters
        ----------
        u : ConceptNode
            The parent node.
        v : ConceptNode
            The child node.
        weight : str, optional
            The key corresponding to the weight parameter in the edge attributes of the layers. 
            By default 'weight'.

        Returns
        -------
        float
            The weight between the parent and child nodes. If these are in different layers, the
            weight is set to 1.

        Raises
        ------
        ValueError
            If nodes are in the same layer, but not connected, a ValueError is raised.
        ValueError
            If nodes have different IDs and are in different layers, a Value Error is raised.
        """
        if u.layer == v.layer:
            G = self.layers[u.layer]
            if not G.has_edge(u.name, v.name):
                raise ValueError(f'Nodes {u.id,v.id} are not connected.')
            # dist = G.edges[(u.name, v.name)][weight]
            dist = G._adj[u.name][v.name][weight]
            if dist is None:
                dist = 1
        elif u.name == v.name:
            dist = 1
        else:
            raise ValueError(f'Nodes {u.id,v.id} are not connected.')
        return dist

    def multidistance(self,
                      path: list[nodeIDType],
                      weight: str = 'weight',
                      initial_dists: MultiDistance | None = None,
                      inplace: bool = False) -> MultiDistance:
        """The multidistance of a path.

        Parameters
        ----------
        path : list[nodeIDType]
            A sequence of node name, node layer tuples defining the path.
        weight : str, optional
            The key corresponding to the weight parameter in the edge attributes of the layers. 
            By default 'weight'.
        initial_dists : MultiDistance, optional
            An initial distance, useful for appending a new path to an existing one, 
            by default MultiDistance(self.n_layers).
        inplace : bool, optional
            Whether initial_dists should be modified in place, by default False.

        Returns
        -------
        MultiDistance
            The multidistance of the specified path.
        """
        if initial_dists is None:
            initial_dists = MultiDistance(self.n_layers)

        if inplace:
            dists = initial_dists
        else:
            dists = deepcopy(initial_dists)

        for ku, kv in pairwise(path):
            u, v = (self.nodes[ku], self.nodes[kv])
            dist = self.edge_weight(u, v, weight=weight)

            key = (u.layer, v.layer)
            dists[key] = dists.get(key, 0) + dist

        return dists

    def multidistance_pair(self,
                           u: nodeIDType,
                           v: nodeIDType,
                           weight: str = 'weight'):
        """The multidistance of the direct path connecting u to v. Assumes this edge exists.

        Parameters
        ----------
        u : nodeIDType
            The name and layer (as a tuple) of the parent node.
        v : nodeIDType
            The name and layer (as a tuple) of the child node.
        weight : str, optional
            The key corresponding to the weight parameter in the edge attributes of the layers. 
            By default 'weight'.

        Returns
        -------
        _type_
            _description_
        """
        md = MultiDistance(self.n_layers)
        md[u[2]] = self.edge_weight(
            self.nodes[u], self.nodes[v], weight=weight)
        return md

    def pareto_shortest_distances(self,
                                  source: ConceptNode,
                                  cut_by_neighbors: bool = False,
                                  depth_cut: int | None = None,
                                  weight: str = 'weight') -> dict[nodeIDType, list[MultiDistance]]:
        """Finds all pareto minimal multidistances from the source node to all reachable nodes.

        Parameters
        ----------
        source : ConceptNode
            The node from which multidistances are computed.
        cut_by_neighbors : bool, optional
            If True, do not consider multidistances that dominate the tuple formed by adding the largest
            distance of each layer between the source and its children, by default False.
        depth_cut : int | None, optional
            An integer specifying a depth at which to terminate the search, by default None.
        weight : str, optional
            The key corresponding to the weight parameter in the edge attributes of the layers. 
            By default 'weight'.

        Returns
        -------
        dict[nodeIDType, list[MultiDistance]]
            A dictionary whose keys are target nodes and whose values are the pareto distance sets from
            the source to each target (subject to the cut_by_neighbors and depth_cut restrictions).
        """

        dist = {}
        seen = {}
        counter = count()
        fringe = []  # (multidistance, counter, conceptnode, depth)

        seen[source.id] = [MultiDistance(self.n_layers)]
        heappush(fringe, ([MultiDistance(self.n_layers)], next(counter), source, 0))

        if cut_by_neighbors:
            neighbor_cut = MultiDistance(self.n_layers)
            for layer, GL in self.layers.items():
                for v in GL.neighbors(source.name):
                    #w = GL.edges[(source.name, v)][weight]
                    w = GL._adj[source.name][v][weight]
                    neighbor_cut[(layer, layer)] = max(
                        neighbor_cut.get((layer, layer), 0), w)

        while fringe:
            (d, _, u, depth) = heappop(fringe)

            if depth_cut is not None:
                if depth > depth_cut:
                    continue

            mm = multimerge(dist.get(u.id, []), d)
            if u.id in dist:
                if all([m in dist[u.id] for m in mm]):
                    continue

            dist[u.id] = mm

            for v in self.neighbors(u):
                uv_dist = [MultiDistance.from_array(x.arr) for x in dist[u.id]]
                # We are making layer crossings free here
                if u.layer == v.layer:
                    w = self.layers[u.layer]._adj[u.name][v.name][weight]
                    lkey = u.layer_ind
                    for pd in uv_dist:
                        pd.add_distance_to_layer(w,lkey)

                if cut_by_neighbors:
                    if any(not(x < neighbor_cut) for x in uv_dist):
                        continue

                # if not in dist, will be set to uv_dist
                v_dist = seen.get(v.id, [])
                #new_vdist = multimin(uv_dist + v_dist)
                new_vdist = multimerge(uv_dist, v_dist)

                if v_dist != new_vdist:
                    seen[v.id] = new_vdist
                    heappush(fringe, (new_vdist, next(counter), v, depth+1))

        return dist

    def all_pairs_pareto_distance(
            self,
            start_layer: str | None = None,
            cut_by_neighbors: bool = False,
            depth_cut: int | None = None,
            weight: str = 'weight') -> Iterator[tuple[Hashable, dict[nodeIDType, list[MultiDistance]]]]:
        """Computes the pareto shortest distsance from each node to each node.

        Parameters
        ----------
        start_layer : str | None, optional
            The layer to use as the source layer. If None (default) all layers are considered; this 
            generally results in redundancies and is not recommended.
        cut_by_neighbors : bool, optional
            If True, do not consider multidistances that dominate the tuple formed by adding the largest
            distance of each layer between the source and its children, by default False.
        depth_cut : int | None, optional
            An integer specifying a depth at which to terminate the search, by default None.
        weight : str, optional
            The key corresponding to the weight parameter in the edge attributes of the layers. 
            By default 'weight'.

        Yields
        ------
        Iterator[tuple[Hashable, dict[nodeIDType, list[MultiDistance]]]]
            Generator that produces a tuple containing a source node and its pareto shortest distances.
        """
        if start_layer is None:
            for n in self.nodes.values():
                yield(n.id, self.pareto_shortest_distances(n,
                                                           cut_by_neighbors=cut_by_neighbors,
                                                           depth_cut=depth_cut,
                                                           weight=weight))
        else:
            for n_name in self.layers[start_layer].nodes():
                n = self.nodes[(n_name, start_layer)]
                yield(n.id, self.pareto_shortest_distances(n,
                                                           cut_by_neighbors=cut_by_neighbors,
                                                           depth_cut=depth_cut,
                                                           weight=weight))

    def pareto_distance_closure(
            self,
            start_layer: str | None = None,
            cut_by_neighbors: bool = False,
            depth_cut: int | None = None,
            weight: str = 'weight') -> dict[Hashable, dict[Hashable, MultiDistance]]:
        """Computes the pareto distance set for each pair of node names.

        Parameters
        ----------
        start_layer : str | None, optional
            The layer to use as the source layer. If None (default) all layers are considered; this 
            generally results in redundancies and is not recommended.
        cut_by_neighbors : bool, optional
            If True, do not consider multidistances that dominate the tuple formed by adding the largest
            distance of each layer between the source and its children, by default False.
        depth_cut : int | None, optional
            An integer specifying a depth at which to terminate the search, by default None.
        weight : str, optional
            The key corresponding to the weight parameter in the edge attributes of the layers. 
            By default 'weight'.

        Returns
        -------
        dict[Hashable, dict[Hashable, MultiDistance]]
            The pareto distance set for each pair of node names.
        """

        return {
            k[0]: {vk[0]: vv for vk, vv in v.items()}
            for k, v in self.all_pairs_pareto_distance(start_layer=start_layer,
                                                       cut_by_neighbors=cut_by_neighbors,
                                                       depth_cut=depth_cut,
                                                       weight=weight)
        }

    def pareto_backbone_removed_edges(
            self,
            closure: dict[Hashable, dict[Hashable,
                                         MultiDistance]] | None = None,
            cut_by_neighbors: bool = False,
            depth_cut: int | None = None,
            weight: str = 'weight') -> set[tuple[nodeIDType, nodeIDType]]:
        """Computes the pareto backbone as a set of edges to remove

        Parameters
        ----------
        closure : dict[Hashable, dict[Hashable, MultiDistance]] | None, optional
            The pareto distance closure from which to compute the backbone. If None (default), it is
            computed using the other input parameters. Otherwise, all other parameters are ignored.
        cut_by_neighbors : bool, optional
            If True, do not consider multidistances that dominate the tuple formed by adding the largest
            distance of each layer between the source and its children, by default False.
        depth_cut : int | None, optional
            An integer specifying a depth at which to terminate the search, by default None.
        weight : str, optional
            The key corresponding to the weight parameter in the edge attributes of the layers. 
            By default 'weight'.

        Returns
        -------
        set[tuple[nodeIDType,nodeIDType]]
            Edges to remove to form the pareto backbone.
        """

        if closure is None:
            closure = self.pareto_distance_closure(cut_by_neighbors=cut_by_neighbors,
                                                   start_layer=self.layer_list[0],
                                                   depth_cut=depth_cut,
                                                   weight=weight)

        edges_to_remove = set()

        for node in self.nodes.values():
            layer = node.layer
            layer_ind = self.layer_ind[layer]
            u = node.name
            G = self.layers[layer]

            for v in G.neighbors(u):
                if cut_by_neighbors:
                    if v in closure[u]:
                        dists = closure[u][v]
                    else:
                        continue
                else:
                    dists = closure[u][v]

                uv_dist = MultiDistance(self.n_layers)
                uv_dist.add_distance_to_layer(G.edges[(u, v)][weight],layer_ind)
                    

                if any([d < uv_dist for d in dists]):
                    edges_to_remove.add(((u, layer), (v, layer)))

        return edges_to_remove

    def weighted_backbone_removed_edges(
            self,
            closure: dict[Hashable, dict[Hashable,
                                         MultiDistance]] | None = None,
            cut_by_neighbors: bool = False,
            depth_cut: int | None = None,
            weight: str = 'weight',
            layer_weights: np.ndarray[np.number] | None = None,
    ) -> set[tuple[nodeIDType, nodeIDType]]:
        """Compute the weighted backbone from the pareto closure using weights for each layer.

        Parameters
        ----------
        closure : dict[Hashable, dict[Hashable, MultiDistance]] | None, optional
            The pareto distance closure from which to compute the backbone. If None (default), it is
            computed using the other input parameters. Otherwise, all other parameters 
            (except layer_weights) are ignored.
        cut_by_neighbors : bool, optional
            If True, do not consider multidistances that dominate the tuple formed by adding the largest
            distance of each layer between the source and its children, by default False.
        depth_cut : int | None, optional
            An integer specifying a depth at which to terminate the search, by default None.
        weight : str, optional
            The key corresponding to the weight parameter in the edge attributes of the layers. 
            By default 'weight'.
        layer_weights : dict[tuple[str, str], float] | None, optional
            Dictionary defining layer weights, by default, layers are weighted by 1.

        Returns
        -------
        set[tuple[nlayer_indodeIDType, nodeIDType]]
            Edges to remove to form the weighted backbone.
        """

        if closure is None:
            closure = self.pareto_distance_closure(weight=weight)

        edges_to_remove = set()

        for node in self.nodes.values():
            layer = node.layer
            layer_ind = self.layer_ind[layer]
            u = node.name
            G = self.layers[layer]

            for v in G.neighbors(u):
                dist = min([sum_pareto_distance(d, layer_weights=layer_weights)
                           for d in closure[u][v]])

                if layer_weights is None:
                    uv_dist = G.edges[(u, v)][weight]
                else:
                    uv_dist = (G.edges[(u, v)][weight]
                               * layer_weights[layer_ind])

                if dist < uv_dist:
                    edges_to_remove.add(((u, layer), (v, layer)))

        return edges_to_remove
