#!/usr/bin/env python3
from __future__ import annotations
from typing import Generator
import pygraphml
import pickle
import sys
import argparse
import os
import abc

def unique_sort(arr: list):
    """
    Args:
        - arr : list

    Returns the unique list of sorted elements
    """
    arr = sorted(arr)
    res = []
    for i in range(len(arr)):
        if i == 0 or (arr[i - 1] != arr[i]):
            res.append(arr[i])
    return res

def full_jaccard(A: list, B: list):
    """
    Args:
        - A: list
        - B: list

    Returns the jaccard index of A and B
    """
    M11, M10, M01 = 0, 0, 0
    elements = unique_sort(A + B)
    for element in elements:
        if element in A:
            if element in B:
                M11 += 1
            else:
                M10 += 1
        else:
            if element in B:
                M01 += 1
    return M11 / (M10 + M01 + M11), M10, M01, M11

def jaccard(A: list, B: list):
    """
    Args:
        - A: list
        - B: list

    Returns the jaccard index of A and B
    """
    M11, M10, M01 = 0, 0, 0
    elements = unique_sort(A + B)
    for element in elements:
        if element in A:
            if element in B:
                M11 += 1
            else:
                M10 += 1
        else:
            if element in B:
                M01 += 1
    return M11 / (M10 + M01 + M11)

def decimalToPercent(number: float) -> str:
    """
    Args:
        - number: float

    Returns percent representation of a decimal
    """
    percent = number * 100
    return "%s%%" % round(percent, ndigits=2)

class Benchmark:
    """Elaboration about Benchmark Results
    """
    def __init__(self, fname: str) -> None:
        self.extract_time(self.parse_csv(self.read_file(fname)))

    def infer_cast(string: str):
        """
        Args:
            - string: str

        Returns native representation of `string`
        """
        try:
            return int(string)
        except:
            pass
        try:
            return float(string)
        except:
            pass
        if string.lower() == "true":
            return True
        if string.lower() == "false":
            return True
        return string

    def read_file(self, path: str) -> str:
        """
        Args:
            - path: str

        Returns content of a file
        """
        with open(path, mode="r", encoding="utf-8") as file:
            return file.read()

    def parse_csv(self, csv_text: str) -> list[dict]:
        """
        Args:
            - csv_text: str

        Returns parsed content of a csv text
        """
        lines = csv_text.strip().split("\n")
        header = [_.strip() for _ in lines[0].strip().split(",")]
        body = []
        for line in lines[1:]:
            fields = [_.strip() for _ in line.strip().split(",")]
            row = {}
            for (field, column) in zip(fields, header):
                row[column] = infer_cast(field)
            body.append(row)
        return body

    def split_csv_by_column(self, csv: list[dict], column: str):
        """
        Args:
            - csv: list[dict]
            - column: str

        Returns a splitted csv object by value of `column` as list of csv objects
        """
        subs = {}
        for line in csv:
            if line[column] not in subs:
                subs[line[column]] = []
            subs[line[column]].append(line)
        return [subs[_] for _ in subs]

    def extract_time_for_repo(self, csv: list[dict]):
        """
        Args:
            - csv: list[dict]

        Computes time info for all records of a repo (aka csv)
        """
        actors = split_csv_by_column(csv, "software")
        for line in actors:
            actor = line[0]["software"]
            repo = line[0]["repo"]
            times = [vals["time"] for vals in line]
            mint, maxt = min(times), max(times)
            midt = sum(times)/len(times)
            vart = sum([pow(midt - x, 2) for x in times])/(len(times)-1)
            print(f"{repo},{actor},{mint},{maxt},{midt}")

    def extract_time(self, csv: list[dict]):
        """
        Args:
            - csv: list[dict]

        Computes time info for all repos
        """
        for repo in split_csv_by_column(csv, "repo"):
            extract_time_for_repo(repo)

class Graph(abc.ABC):
    """Graph Representation
    """
    def __init__(self, fname: str, clean: bool = False) -> None:
        """
        Args:
            - fname: str
            - clean: bool
        
        If (`clean` == True) then don't use cached pickle file;
        Else If (cached file exists) then use it;
        Else build the graph and write the cached file.
        """
        cfname = fname.replace("graphml", "pickle")
        if os.path.exists(cfname) and not clean:
            self.load_from_pickle(cfname)
        else:
            self.load_from_graphml(fname)
            self.write_to_pickle(cfname)
    def edges(self) -> Generator[tuple, None, None]:
        """
        Returns a generator for edges (aka tuple[source: str, sink: str])
        """
        for node in self.nodes():
            for dependency in node["dependencies"]:
                yield (node["name"], dependency)
    def nodes(self) -> Generator[dict, None, None]:
        """
        Returns a generator for nodes (aka dict)
        """
        for name in self._nodes:
            yield self._nodes[name]
    def write_to_pickle(self, fname: str):
        with open(fname, mode="wb") as file:
            pickle.dump(self._nodes, file)
    def load_from_pickle(self, fname: str):
        with open(fname, mode="rb") as file:
            self._nodes = pickle.load(file)
    @abc.abstractmethod
    def edge_kind_is_standard(self, kind: str) -> bool:
        """
        Args:
            - kind: str

        Returns true if the edge `kind` specified is considered standard
        """
    @abc.abstractmethod
    def node_kind_is_standard(self, kind: str) -> bool:
        """
        Args:
            - kind: str

        Returns true if the node `kind` specified is considered standard
        """
    def add_dependency(self, source: str, sink: str):
        if sink not in self._nodes[source]["dependencies"]:
            self._nodes[source]["dependencies"].append(sink)
    def remove_self_dependencies(self):
        """
        Optionally removes self dependencies (aka S : node,  S -> S)
        Returns True if graph got modified
        """
        modified = False
        for node in self.nodes():
            if node["name"] in node["dependencies"]:
                node["dependencies"].remove(node["name"])
                modified = True
        return modified

class SkullianGraph(Graph):
    """Skullian Graph Representation
    
    nodes:
     - annotation
     - annotationElement
     - attribute
     - class
     - constant
     - enum
     - enumVariant
     - function
     - interface
     - package
     - parameter
    """
    def edge_kind_is_standard(self, kind: str) -> bool:
        return True
    def node_kind_is_standard(self, kind: str) -> bool:
        return kind in [
            "annotation",
            "class",
            "enum",
            "interface",
            #"package"
        ]
    def load_from_graphml(self, fname: str):
        graphml = pygraphml.graphml_parser.GraphMLParser().parse(fname)
        # Process Nodes
        self._nodes = {}
        for node in graphml.nodes():
            name = node.attributes()["qualified_name"].value.strip()
            kind = node.attributes()["kind"].value.strip()
            if self.node_kind_is_standard(kind):
                self._nodes[name] = {
                        "name": name,
                        "dependencies": []
                }
        # Build Parent Mesh
        parents = {}
        for edge in graphml.edges():
            kind = edge.attributes()["relationship"].value.strip()
            if kind in ["definedBy"]:
                source = edge.node1.attributes()["qualified_name"].value.strip()
                sink = edge.node2.attributes()["qualified_name"].value.strip()
                parents[source] = sink
            if kind in ["nestedTo"]:
                source = edge.node1.attributes()["qualified_name"].value.strip()
                if source in self._nodes:
                    self._nodes.pop(source)
        # Process Edges
        for edge in graphml.edges():
            kind = edge.attributes()["relationship"].value.strip()
            source = edge.node1.attributes()["qualified_name"].value.strip()
            sink = edge.node2.attributes()["qualified_name"].value.strip()
            if kind in ["definedBy", "nestedTo"]:
                #if source in self._nodes and sink in self._nodes:
                #    self.add_dependency(source, sink)
                pass
            else:
                ok = True
                while source not in self._nodes:
                    if source not in parents:
                        ok = False
                        break
                    source = parents[source]
                while sink not in self._nodes:
                    if sink not in parents:
                        ok = False
                        break
                    sink = parents[sink]
                if ok:
                    self.add_dependency(source, sink)

class ArcanGraph(Graph):
    """Arcan Graph Representation

    nodes:
     - ABSTRACT_CLASS
     - ANNOTATION
     - CONCRETE_CLASS
     - ENUM
     - INTERFACE
     - PACKAGE
    relationships:
     - unitIsAfferentOf
     - dependsOn
     - belongsTo
     - containerIsAfferentOf
     - isChildOf
     - isImplementationOf
    """
    def edge_kind_is_standard(self, kind: str) -> bool:
        return kind in [
            "dependsOn",
            #"belongsTo",
            "isChildOf",
            "isImplementationOf"
        ]
    def node_kind_is_standard(self, kind: str) -> bool:
        return kind in [
            "ABSTRACT_CLASS",
            "ANNOTATION",
            "CONCRETE_CLASS",
            "ENUM",
            "INTERFACE",
            #"PACKAGE"
        ]
    def load_from_graphml(self, fname: str):
        graphml = pygraphml.graphml_parser.GraphMLParser().parse(fname)
        # Process Nodes
        self._nodes = {}
        for node in graphml.nodes():
            name = node.attributes()["name"].value.strip()
            kind = node.attributes()["constructType"].value.strip()
            if self.node_kind_is_standard(kind):
                self._nodes[name] = {
                        "name": name,
                        "dependencies": []
                }
        # Process Edges
        for edge in graphml.edges():
            kind = edge.attributes()["labelE"].value.strip()
            source = edge.node1.attributes()["name"].value.strip()
            sink = edge.node2.attributes()["name"].value.strip()
            if self.edge_kind_is_standard(kind):
                if source in self._nodes and sink in self._nodes:
                    self.add_dependency(source, sink)

def mkcli():
    cli = argparse.ArgumentParser(prog="main")
    cli.add_argument("target", type=str, nargs='?')
    cli.add_argument("--benchmark", "-B", action="store_true", help="operate on Benchmark")
    cli.add_argument("--graphml", "-G", action="store_true", help="operate on GraphML")
    cli.add_argument("--clean", "-c", action="store_true", help="don't use cached files")
    cli.add_argument("--noself", "-s", action="store_true", help="remove self dependencies")
    cli.add_argument("--jaccard", "-j", action="store_true", help="show Jaccard indexes")
    cli.add_argument("--print-arcan", "-a", action="store_true", help="show Arcan edges")
    cli.add_argument("--print-skullian", "-p", action="store_true", help="show Skullian edges")
    return cli.parse_args(sys.argv[1:])

def do_benchmark(cli):
    target = "benchmark.csv"
    if cli.target and cli.target != "":
        target = cli.target
    benchmark = Benchmark(target)

def do_graphml(cli):
    project = ""
    if cli.target and cli.target != "":
        project = "_" + cli.target
    skullian = SkullianGraph("skullian%s.graphml" % project, clean=cli.clean)
    arcan = ArcanGraph("arcan%s.graphml" % project, clean=cli.clean)
    if cli.noself:
        if arcan.remove_self_dependencies():
            print("arcan had self dependencies")
        if skullian.remove_self_dependencies():
            print("skullian had self dependencies")

    if cli.jaccard:
        arcan_nodes = [node["name"] for node in arcan.nodes()]
        arcan_edges = [" -> ".join(edge) for edge in arcan.edges()]
        arcan_graph = arcan_nodes + arcan_edges

        skullian_nodes = [node["name"] for node in skullian.nodes()]
        skullian_edges = [" -> ".join(edge) for edge in skullian.edges()]
        skullian_graph = skullian_nodes + skullian_edges

        jn, n10, n01, n11 = full_jaccard(arcan_nodes, skullian_nodes)
        je, e10, e01, e11 = full_jaccard(arcan_edges, skullian_edges)
        jg, g10, g01, g11 = full_jaccard(arcan_graph, skullian_graph)
        jn, je, jg = decimalToPercent(jn), decimalToPercent(je), decimalToPercent(jg)
        print("%s & %s & %s & %s & %s & %s & %s \\\\ \\hline" % (project, jn, (n10, n01, n11), je, (e10, e01, e11), jg, (g10, g01, g11)))

    if cli.print_arcan:
        for edge in arcan.edges():
            print(" -> ".join(edge))

    if cli.print_skullian:
        for edge in skullian.edges():
            print(" -> ".join(edge))

if __name__ == "__main__":
    cli = mkcli()
    if cli.benchmark:
        do_benchmark(cli)
    elif cli.graphml:
        do_graphml(cli)
