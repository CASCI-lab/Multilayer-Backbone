# type: ignore
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


ER0 = nx.erdos_renyi_graph(15, 0.05)
ER1 = nx.erdos_renyi_graph(15, 0.04)
ER2 = nx.erdos_renyi_graph(15, 0.03)
ER3 = nx.erdos_renyi_graph(15, 0.1)
ER4 = nx.erdos_renyi_graph(15, 0.1)
ER5 = nx.erdos_renyi_graph(15, 0.1)

graphs = [ER0, ER1, ER2, ER3, ER4, ER5]
# graphs = [ER0]
for ER in graphs:
    for u, v in ER.edges():
        ER.edges[(u, v)]["weight"] = u + v
edgelist = []
interweight = 0.000

index = 0
for i, G in enumerate(graphs):
    for node in G.nodes:
        G.nodes[node]["index"] = index
        index += 1

        for j, Gp in enumerate(graphs[0:i]):
            if node in Gp.nodes:
                edgelist.append((index, Gp.nodes[node]["index"], i, j, 0, interweight))
                edgelist.append((Gp.nodes[node]["index"], index, j, i, 0, interweight))

    for u, v, d in G.edges(data=True):
        edgelist.append(
            (G.nodes[u]["index"], G.nodes[v]["index"], 0, 0, 0, d["weight"])
        )

# print(edgelist)

# closure = bb.distance_closure_py(edgelist)
# print(closure)
multilayer_backbone = bb.backbone_py(edgelist)
