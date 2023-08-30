# type: ignore
import random  # noqa
import time

import backbone as bb
import networkx as nx

# G_L0 = nx.DiGraph()
# G_L1 = nx.DiGraph()
# G_L2 = nx.DiGraph()
# G_L0.add_edges_from(
#     [
#         ("A", "B", dict(weight=1.0)),
#         ("A", "E", dict(weight=5.0)),
#         ("A", "D", dict(weight=1.0)),
#         ("B", "C", dict(weight=1.0)),
#         ("D", "E", dict(weight=1.0)),
#         ("C", "E", dict(weight=1.0)),
#         ("E", "F", dict(weight=1.0)),
#     ]
# )
# G_L1.add_edges_from(
#     [
#         ("A", "C", dict(weight=1.0)),
#         ("C", "B", dict(weight=1.0)),
#         ("B", "D", dict(weight=1.0)),
#         ("B", "F", dict(weight=1.0)),
#         ("D", "E", dict(weight=1.0)),
#     ]
# )
# G_L2.add_edges_from(
#     [
#         ("A", "B", dict(weight=1.0)),
#         ("C", "D", dict(weight=1.0)),
#         ("E", "F", dict(weight=1.0)),
#     ]
# )
# graphs = [G_L0, G_L1, G_L2]


ER0 = nx.erdos_renyi_graph(50, 0.05)
ER1 = nx.erdos_renyi_graph(10, 0.01)
ER2 = nx.erdos_renyi_graph(25, 0.03)
ER3 = nx.erdos_renyi_graph(25, 0.1)
ER4 = nx.erdos_renyi_graph(15, 0.1)
ER5 = nx.erdos_renyi_graph(20, 0.1)
graphs = [ER0, ER1, ER2, ER3, ER4, ER5]
for ER in graphs:
    for u, v in ER.edges():
        ER.edges[(u, v)]["weight"] = u + v


# ER6 = nx.erdos_renyi_graph(50, 0.1)
# graphs = [ER6]
# for ER in graphs:
#     for u, v in ER.edges():
#         ER.edges[(u, v)]["weight"] = u + v


# G0 = nx.DiGraph()
# G1 = nx.DiGraph()
# G0.add_edges_from(
#     [
#         ("A", "B", dict(weight=3.0)),
#         ("A", "C", dict(weight=1.0)),
#         ("C", "B", dict(weight=1.0)),
#     ]
# )
# G1.add_edges_from(
#     [
#         ("A", "B", dict(weight=1.0)),
#         ("A", "C", dict(weight=1.0)),
#         ("C", "B", dict(weight=1.0)),
#     ]
# )
# graphs = [G0, G1]

edgelist = []
interweight = 0.00

index_lookup = {}

index = 0
for i, G in enumerate(graphs):
    for node in G.nodes:
        G.nodes[node]["index"] = index
        index_lookup[index] = f"{node}_{i}"

        for j, Gp in enumerate(graphs[0:i]):
            if node in Gp.nodes:
                edgelist.append((index, Gp.nodes[node]["index"], i, j, 0, interweight))
                edgelist.append((Gp.nodes[node]["index"], index, j, i, 0, interweight))
        index += 1
    for u, v, d in G.edges(data=True):
        edgelist.append(
            (G.nodes[u]["index"], G.nodes[v]["index"], i, i, 0, d["weight"])
        )


def test_edge_list(edgelist, method, verbose=False):
    t0 = time.perf_counter_ns()
    multilayer_backbone = method(edgelist)
    t1 = time.perf_counter_ns()
    bb_edges = set()
    for u, d in sorted(multilayer_backbone.items()):
        try:
            edges = sorted(d.items())
        except AttributeError:
            edges = sorted(d)
        for v, weight in edges:
            if verbose:
                print(index_lookup[u], index_lookup[v], weight)
            bb_edges.add((u, v))
    return t1 - t0, bb_edges


verbose = False

print("=" * 20)
print("BACKBONE (SIMAS)")
print("- " * 10)
ts, bb_simas_edges = test_edge_list(
    edgelist, bb.structural_backbone_simas, verbose=verbose
)

print("=" * 20)
print("BACKBONE (COSTA)")
print("- " * 10)
tc, bb_costa_edges = test_edge_list(
    edgelist, bb.structural_backbone_costa, verbose=verbose
)

print("=" * 20)
print("BACKBONE (NAÏVE)")
print("- " * 10)
tn, bb_naive_edges = test_edge_list(
    edgelist, bb.structural_backbone_naive, verbose=verbose
)

print("=" * 20)
print("BACKBONE (CLOSURE)")
print("- " * 10)
tb, bb_from_closure_edges = test_edge_list(edgelist, bb.backbone_py, verbose=verbose)

print("=-" * 20)
print("-=" * 20)
if bb_simas_edges == bb_costa_edges == bb_naive_edges == bb_from_closure_edges:
    print("Backbones match!")
else:
    print("BACKBONES DON'T MATCH!!!")
    print(f"{(bb_simas_edges == bb_costa_edges)=}")
    print(f"{(bb_simas_edges == bb_naive_edges)=}")
    print(f"{(bb_simas_edges == bb_from_closure_edges)=}")
    print(f"{(bb_costa_edges == bb_naive_edges)=}")
    print(f"{(bb_costa_edges == bb_from_closure_edges)=}")
    print(f"{(bb_naive_edges == bb_from_closure_edges)=}")

print(f"Simas backbone took {ts*1e-9:.5e}s")
print(f"Costa backbone took {tc*1e-9:.5e}s")
print(f"Naïve backbone took {tn*1e-9:.5e}s")
print(f"Closure backbone took {tb*1e-9:.5e}s")
