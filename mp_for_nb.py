def appd(arg):
    G = arg[0]
    n = arg[1]
    return n.id, G.pareto_shortest_distances(n,depth_cut=None)

def make_arglist(KG,layer):
    return [(KG,KG.nodes[(n_name, layer)]) for n_name in KG.layers[layer].nodes()]