import networkx as nx
import heapq as hq
from copy import deepcopy

import KnowledgeGraph as kg  
import ConceptNode as cn 
from MultiDistance import MultiDistance, multimin, sum_pareto_distance

ER0 = nx.erdos_renyi_graph(100, 0.7)
ER1 = nx.erdos_renyi_graph(100, 0.7)
ER2 = nx.erdos_renyi_graph(100, 0.7)
ER3 = nx.erdos_renyi_graph(100, 0.7)
ER4 = nx.erdos_renyi_graph(100, 0.7)
ER5 = nx.erdos_renyi_graph(100, 0.7)

layersER = {
    'L0': ER0,
    'L1':ER1,
    #'L2':ER2,
    #'L3':ER3,
    # 'L4':ER4,
    # 'L5':ER5,
}


for ER in layersER.values():
    for (u, v) in ER.edges():
        ER.edges[(u, v)]['weight'] = u+v


ERKG = kg.KnowledgeGraph(layersER)

list(ERKG.all_pairs_pareto_distance(start_layer='L0',depth_cut=3))