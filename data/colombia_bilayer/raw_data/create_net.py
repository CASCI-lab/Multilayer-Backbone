# -*- coding: utf-8 -*-
"""
Created on Tue Jan 24 15:27:57 2023

@author: Felipe Xavier Costa
"""

import pandas as pd
import networkx as nx
import numpy as np
import os
from distanceclosure.utils import prox2dist

pd.set_option('display.max_rows', 100)
pd.set_option('display.max_columns', 500)
pd.set_option('display.width', 1000)

def create_network(filename, metadata):
    edge_list = pd.read_table(filename, header=None, names=['i', 'j', 'count'])
        
    G = nx.from_pandas_edgelist(edge_list, source='i', target='j',
                                edge_attr=['count'], create_using=nx.DiGraph)
    
    nx.set_node_attributes(G, metadata.to_dict(), name='city_name')
    
    k = G.out_degree(weight='count')
    P_dict = {(u, v): c/k[u] for u, v, c in G.edges(data='count')}
    nx.set_edge_attributes(G, name='proximity', values=P_dict)
    
    # Remove self-loops
    print('Remove self-loops')
    G.remove_edges_from(list(nx.selfloop_edges(G)))

    # Keep only largest connected component
    print('Keep largest connected component')
    G = G.subgraph(max(nx.weakly_connected_components(G), key=len)).copy()
    
    print('Prox -> Dist')
    D_dict = {key: prox2dist(value) for key, value in P_dict.items()}
    nx.set_edge_attributes(G, name='distance', values=D_dict)
    
    return G

## City Names

code_names = pd.read_table('code_municipio', header=None, index_col=0)

print('Calls Network')
G = create_network('city_city_nedges_calls', metadata=code_names)


if not os.path.exists('calls'):
    os.mkdir('calls')
nx.write_graphml(G, 'calls/network.graphml')


print('Mobility Network')
G = create_network('city_city_nedges_mobility', metadata=code_names)

if not os.path.exists('mobility'):
    os.mkdir('mobility')
nx.write_graphml(G, 'mobility/network.graphml')    