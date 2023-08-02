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

edgelist = []
interweight = 0.000

index = 0
for node in G_L0.nodes:
    G_L0.nodes[node]["index"] = index
    index += 1
for node in G_L1.nodes:
    G_L1.nodes[node]["index"] = index
    index += 1
    if node in G_L0.nodes:
        edgelist.append((index, G_L0.nodes[node]["index"], 1, 0, 0, interweight))
        edgelist.append((G_L0.nodes[node]["index"], index, 0, 1, 0, interweight))
for node in G_L2.nodes:
    G_L2.nodes[node]["index"] = index
    index += 1

    if node in G_L0.nodes:
        edgelist.append((index, G_L0.nodes[node]["index"], 2, 0, 0, interweight))
        edgelist.append((G_L0.nodes[node]["index"], index, 0, 2, 0, interweight))
    if node in G_L1.nodes:
        edgelist.append((index, G_L1.nodes[node]["index"], 2, 1, 0, interweight))
        edgelist.append((G_L1.nodes[node]["index"], index, 1, 2, 0, interweight))

for u, v, d in G_L0.edges(data=True):
    edgelist.append(
        (G_L0.nodes[u]["index"], G_L0.nodes[v]["index"], 0, 0, 0, d["weight"])
    )
for u, v, d in G_L1.edges(data=True):
    edgelist.append(
        (G_L1.nodes[u]["index"], G_L1.nodes[v]["index"], 1, 1, 0, d["weight"])
    )
for u, v, d in G_L2.edges(data=True):
    edgelist.append(
        (G_L2.nodes[u]["index"], G_L2.nodes[v]["index"], 2, 2, 0, d["weight"])
    )

print(edgelist)

closure = bb.distance_closure_py(edgelist)
print(closure)
