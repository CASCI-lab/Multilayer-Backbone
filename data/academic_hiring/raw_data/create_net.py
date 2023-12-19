# -*- coding: utf-8 -*-
"""
Created on Mon Jan 23 13:29:36 2023

@author: Felipe Xavier Costa
"""

import pandas as pd
import networkx as nx
import os
from distanceclosure.utils import prox2dist

def create_graph(filename):
    
    print('Loading File')
    edge_data = pd.read_csv(filename, delimiter='\t')
    edge_data.rename(columns={'# u': 'u'}, inplace=True)
    
    edge_attr = edge_data.groupby(['u', 'v'], as_index=False)["gender"].count()
    edge_attr.rename(columns={'gender': 'NFaculty'}, inplace=True)
    
    print('Connecting Nodes')
    
    G = nx.from_pandas_edgelist(edge_attr, source='u', target='v', 
                                edge_attr=['NFaculty'], create_using=nx.DiGraph)
    
    print('Counts -> Prox')
    k = G.out_degree(weight='NFaculty')
    P_dict = {(u, v): c/k[u] for u, v, c in G.edges(data='NFaculty')}
    nx.set_edge_attributes(G, name='proximity', values=P_dict)
    
    print('Prox -> Dist')
    D_dict = {key: prox2dist(value) for key, value in P_dict.items()}
    nx.set_edge_attributes(G, name='distance', values=D_dict)
    
    #print(list(nx.selfloop_edges(G)))
    print('Remove self-loops')
    G.remove_edges_from(list(nx.selfloop_edges(G)))

    # Keep only largest connected component
    print('Keep largest connected component')
    G = G.subgraph(max(nx.weakly_connected_components(G), key=len)).copy()
    
    return G

def add_metadata(G, filename):
    node_data = pd.read_csv(filename, delimiter='\t')
    node_data.index = node_data['# u']
    
    node_attr = node_data.to_dict()
    nx.set_node_attributes(G, node_attr['Region  '], name='Region')
    nx.set_node_attributes(G, node_attr['institution'], name='Institution')
    
    return G


if __name__ == '__main__':

    ### Business Network
    G = create_graph('./Datasets/Business_edgelist.txt')
    G = add_metadata(G, './Datasets/Business_vertexlist.txt')
    G.name = 'Business Faculty US'

    if not os.path.exists('business'):
        os.mkdir('business')
    nx.write_graphml(G, 'business/network.graphml')

    ### Computer Science Network
    G = create_graph('./Datasets/ComputerScience_edgelist.txt')
    G = add_metadata(G, './Datasets/ComputerScience_vertexlist.txt')
    G.name = 'Computer Science Faculty US'

    if not os.path.exists('computer_science'):
        os.mkdir('computer_science')
    nx.write_graphml(G, 'computer_science/network.graphml')

    ### History Network
    G = create_graph('./Datasets/History_edgelist.txt')
    G = add_metadata(G, './Datasets/History_vertexlist.txt')
    G.name = 'History Faculty US'

    if not os.path.exists('history'):
        os.mkdir('history')
    nx.write_graphml(G, 'history/network.graphml')

