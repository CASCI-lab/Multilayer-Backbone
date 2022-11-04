def appd(arg):
    G = arg[0]
    n = arg[1]
    return n.id, G.pareto_shortest_distances(n,depth_cut=None)

def appd_cut(arg):
    G = arg[0]
    n = arg[1]
    c = arg[2]
    return n.id, G.pareto_shortest_distances(n,depth_cut=c)

def make_arglist(KG,layer,cut=None):
    if cut:
        return [(KG,KG.nodes[(n_name, layer)],cut) for n_name in KG.layers[layer].nodes()]
    else:
        return [(KG,KG.nodes[(n_name, layer)]) for n_name in KG.layers[layer].nodes()]