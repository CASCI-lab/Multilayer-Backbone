# type: ignore
import time

import backbone as bb
import networkx as nx

G_L0 = nx.DiGraph()
G_L1 = nx.DiGraph()
G_L2 = nx.DiGraph()

G_L0.add_edges_from(
    [
        ("A", "B", dict(weight=1.0)),
        ("A", "E", dict(weight=5.0)),
        ("A", "D", dict(weight=1.0)),
        ("B", "C", dict(weight=1.0)),
        ("D", "E", dict(weight=1.0)),
        ("C", "E", dict(weight=1.0)),
        ("E", "F", dict(weight=1.0)),
    ]
)

G_L1.add_edges_from(
    [
        ("A", "C", dict(weight=1.0)),
        ("C", "B", dict(weight=1.0)),
        ("B", "D", dict(weight=1.0)),
        ("B", "F", dict(weight=1.0)),
        ("D", "E", dict(weight=1.0)),
    ]
)

G_L2.add_edges_from(
    [
        ("A", "B", dict(weight=1.0)),
        ("C", "D", dict(weight=1.0)),
        ("E", "F", dict(weight=1.0)),
    ]
)

graphs = [G_L0, G_L1, G_L2]


ER0 = nx.erdos_renyi_graph(50, 0.05)
ER1 = nx.erdos_renyi_graph(10, 0.01)
ER2 = nx.erdos_renyi_graph(25, 0.03)
ER3 = nx.erdos_renyi_graph(25, 0.1)
ER4 = nx.erdos_renyi_graph(15, 0.1)
ER5 = nx.erdos_renyi_graph(20, 0.1)
graphs = [ER0, ER1, ER2, ER3, ER4, ER5]

# ER6 = nx.erdos_renyi_graph(1500, 0.05)
# graphs = [ER6]
for ER in graphs:
    for u, v in ER.edges():
        ER.edges[(u, v)]["weight"] = u + v


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
interweight = 0.000

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

# G_L0 = nx.read_graphml("examples/network_yeast.graphml")
# for i, u in enumerate(G_L0.nodes):
#     G_L0.nodes[u]["index"] = i
# edgelist = []
# for u, v, d in G_L0.edges(data=True):
#     if d["distance"] > 0:
#         edgelist.append(
#             (G_L0.nodes[u]["index"], G_L0.nodes[v]["index"], 0, 0, 0, d["distance"])
#         )

print("=" * 20)
print("BACKBONE (STRUCTURAL)")
print("- " * 10)
t0s = time.perf_counter_ns()
multilayer_backbone_structural = bb.structural_backbone_simas(edgelist)
t1s = time.perf_counter_ns()
bb_from_structure_edges = set()
for u, d in sorted(multilayer_backbone_structural.items()):
    for v, weight in sorted(d):
        # print(index_lookup[u], index_lookup[v], weight)
        bb_from_structure_edges.add((u, v))

print("=" * 20)
print("BACKBONE (CLOSURE)")
print("- " * 10)
t0b = time.perf_counter_ns()
multilayer_backbone = bb.backbone_py(edgelist)
t1b = time.perf_counter_ns()
bb_from_closure_edges = set()
for u, d in sorted(multilayer_backbone.items()):
    for v, weight in sorted(d.items()):
        # print(index_lookup[u], index_lookup[v], weight)
        bb_from_closure_edges.add((u, v))


print("=-" * 20)
print("-=" * 20)
if bb_from_closure_edges == bb_from_structure_edges:
    print("Backbones match!")
else:
    print("Backbones don't match!")

print(f"Backbone from closure took {t1b-t0b:.5E}ns (includes closure)")
print(f"Structural backbone took {t1s-t0s:.5E}ns")
