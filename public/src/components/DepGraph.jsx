import React, { Component } from 'react';
import CytoscapeComponent from 'react-cytoscapejs';
import { saveAs } from 'file-saver';

class DepGraph extends Component {
    constructor(props) {
        super(props);
	this.state = {};
    }

    assignCy = (cy) => {
	if (this.state.cy === undefined) {
	    this.setState({cy: cy});
        }
    }

    saveAsPng = () => {
        let blob = this.state.cy.png();
	saveAs(blob, "graph.png");
    }

    render() {
        return (
            <div>
                <button onClick={this.saveAsPng}>save as png</button>
                <CytoscapeComponent
                    cy={this.assignCy}
                    elements={this.props.elements}
                    style={{
                        width: window.innerWidth,
                        height: window.innerHeight
                    }}
                    stylesheet={[
                        {
                            selector: 'edge',
                            style: {
                              width: 4,
                              targetArrowShape: 'triangle',
                              curveStyle: 'bezier',
                              label: 'data(label)'
                            }
                        },
                        {
                            selector: 'node',
                            style: {
                                shape: 'hexagon',
                                'background-color': 'blue',
                                label: 'data(id)',
                            }
                        }
                    ]}
                    layout={this.props.layout}
                />
            </div>
        );
    }
}

export default DepGraph;
