import cytoscapeAvsdf from 'cytoscape-avsdf';
//import { cytoscapeCise } from 'cytoscape-cise';
//import { cytoscapeCoseBilkent } from 'cytoscape-cose-bilkent';
import cytoscapeFcose from 'cytoscape-fcose';
import cytoscapeKlay from 'cytoscape-klay';
//import { cytoscapeNoOverlap } from 'cytoscape-no-overlap';
import React, { Component } from 'react';
import { cytoscape } from 'react-cytoscape';
import DepGraph from '../components/DepGraph';

cytoscape.use(cytoscapeKlay);
cytoscape.use(cytoscapeFcose);
cytoscape.use(cytoscapeAvsdf);/*
cytoscape.use(cytoscapeCoseBilkent);
cytoscape.use(cytoscapeCise);
cytoscape.use(cytoscapeNoOverlap);*/

class Home extends Component {
    constructor(props) {
        super(props);
        this.state = {
            layout: "random",
            layouts: [ "random", "breadthfirst", "fcose", "klay", "avsdf"
            ],
            layoutsData: {
                "random": {
                    name: "random"
                },
                "breadthfirst": {
                    name: "breadthfirst"
                },
                "avsdf": {
                    name: "avsdf",
                    nodeSeparation: 200
                },
                "fcose": {
                    name: "fcose",
                    idealEdgeLength: edge => 200
                },
                "klay": {
                    name: "klay",
                    "klay": {spacing: 200}
                }
            },
            elements: []
        }
    }

    componentDidMount() {
	fetch("/js/data.json")
	.then(data => data.json())
	.then(json => this.setState({elements: json}));
    }

    setLayout = (layout) => {
        this.setState({
            layout: layout
        });
    }

    render() {
	let buttons = this.state.layouts.map(layout => {
                    return (
                        <button
                            key={layout}
                            onClick={(e) => this.setLayout(layout)}
                        >
                            set {layout}
                        </button>
                    )
                });
	if (this.state.elements.length == 0) {
		return (
		    <div>
			{buttons}
		    </div>
		);
	} else {
		return (
		    <div>
			{buttons}
			<DepGraph
			    elements={this.state.elements}
			    layout={this.state.layoutsData[this.state.layout]}
			/>
		    </div>
		);
	}
    }
}

export default Home;
