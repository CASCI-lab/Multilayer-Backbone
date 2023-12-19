# Phone calls and mobility networks

Source: <https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0145091>

Files used:
- `city_city_nedges_calls`: Calls network data
- `city_city_nedges_mobility`: Mobility network data
- `code_municipio`: metadata

We have two networks among Colombian municipalitites based on phone calls and phone mobility.

To construct the mobility network, the authors have considered to which tower the phone is connected and its corresponding changes.
In this case, the edge weights between municipalities is the amount of flow from one to another.

For the phone calls network, we have edges weighted by the number of calls from one municipality to another.
