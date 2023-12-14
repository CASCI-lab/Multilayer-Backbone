# type: ignore
from itertools import combinations
from numbers import Number
from typing import Hashable, Iterable, cast

import distanceclosure as dc
import networkx as nx


def multidistance_backbone(
    multilayer_graph: nx.MultiDiGraph,
    weight_edge_attribute: str = "weight",
    layer_edge_attribute: str = "layer",
    kind: str = "metric",
    self_loops: bool = False,
) -> nx.MultiDiGraph:
    backbone = nx.MultiDiGraph()

    layer_slices: dict[Hashable, nx.DiGraph] = {}
    zero_weight_subgraph = nx.DiGraph()
    for u, v, d in cast(
        Iterable[tuple[Hashable, Hashable, dict[Hashable, float | Hashable]]],
        multilayer_graph.edges(data=True),
    ):
        layer = d[layer_edge_attribute]
        weight = d[weight_edge_attribute]
        if not isinstance(weight, Number):
            raise ValueError(f"Weight must be a number, not {type(weight).__name__}")
        if weight < 0:
            raise ValueError(f"Weight must be non-negative, not {weight}")
        if layer not in layer_slices:
            layer_slices[layer] = nx.DiGraph()

        layer_slices[layer].add_edge(u, v, **d)
        if weight == 0:
            zero_weight_subgraph.add_edge(u, v)

    zero_weight_subgraph_closure = cast(
        nx.DiGraph,
        nx.transitive_closure(zero_weight_subgraph, reflexive=False),
    )

    new_zero_weight_edges = set(zero_weight_subgraph_closure.edges()) - set(
        zero_weight_subgraph.edges()
    )

    for u in zero_weight_subgraph_closure.nodes():
        for v in zero_weight_subgraph.neighbors(u):
            for layer, subgraph in layer_slices.items():
                if u in subgraph and v in subgraph:
                    subgraph.add_edge(u, v, weight=0, layer=layer)

    for layer, subgraph in layer_slices.items():
        layer_backbone = dc.backbone(subgraph, kind=kind, self_loops=self_loops)

        backbone.add_edges_from(
            (u, v, d | subgraph.get_edge_data(u, v))
            for u, v, d in layer_backbone.edges(data=True)
            if (u, v) not in new_zero_weight_edges
        )

    return backbone


def combine_graphs(
    graphs: list[nx.MultiDiGraph],
    graph_labels: list[Hashable],
    rename_nodes: bool = False,
    identity_edge_weight: float | None = None,
    additional_edges: list[tuple[Hashable, Hashable, tuple[Hashable, Hashable], float]]
    | None = None,
    weight_edge_attribute: str = "weight",
    layer_edge_attribute: str = "layer",
) -> nx.MultiDiGraph:
    multilayer_graph = nx.MultiDiGraph()
    if additional_edges is None:
        additional_edges = []

    if identity_edge_weight is not None:
        for (g1, l1), (g2, l2) in combinations(zip(graphs, graph_labels), 2):
            for u, v, d in g1.edges(data=True):
                if u in g2.nodes and v in g2.nodes:
                    additional_edges.append((u, v, (l1, l2), identity_edge_weight))
            for u, v, d in g2.edges(data=True):
                if u in g1.nodes and v in g1.nodes:
                    additional_edges.append((u, v, (l2, l1), identity_edge_weight))

    for graph, label in zip(graphs, graph_labels):
        if rename_nodes:
            nx.relabel_nodes(graph, lambda n: f"{n}_{label}", copy=False)

        for u, v, d in graph.edges(data=True):
            d_new = d.copy()
            d_new[layer_edge_attribute] = (label, label)
            multilayer_graph.add_edge(u, v, **d_new)
    if rename_nodes:
        additional_edges = [
            (f"{u}_{l1}", f"{v}_{l2}", (l1, l2), w)
            for u, v, (l1, l2), w in additional_edges
        ]
    for u, v, layer, weight in additional_edges:
        d = {}
        d[weight_edge_attribute] = weight
        d[layer_edge_attribute] = layer
        multilayer_graph.add_edge(u, v, **d)

    return multilayer_graph


if __name__ == "__main__":
    graph0 = nx.DiGraph()
    graph1 = nx.DiGraph()

    graph0.add_edge(0, 1, weight=1, dummy="123")
    graph0.add_edge(1, 2, weight=1)
    graph0.add_edge(2, 0, weight=1)
    graph0.add_edge(0, 2, weight=3)

    graph1.add_edge(0, 1, weight=1)
    graph1.add_edge(1, 2, weight=1)
    graph1.add_edge(2, 0, weight=1)
    graph1.add_edge(0, 2, weight=1)

    mg = combine_graphs([graph0, graph1], [0, 1])
    for x in mg.edges(data=True):
        print(x)
    print("-" * 20)
    mgr = combine_graphs(
        [graph0, graph1],
        [0, 1],
        rename_nodes=True,
        additional_edges=[(0, 2, (0, 1), 10)],
    )
    for x in mgr.edges(data=True):
        print(x)
    print("-" * 20)

    bb = multidistance_backbone(mg)
    for x in bb.edges(data=True):
        print(x)
    print("-" * 20)

    bbr = multidistance_backbone(mgr)
    for x in bbr.edges(data=True):
        print(x)
    print("-" * 20)
