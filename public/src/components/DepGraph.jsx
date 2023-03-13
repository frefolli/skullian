import React, { Component } from 'react';
import CytoscapeComponent from 'react-cytoscapejs';

class DepGraph extends Component {
    constructor(props) {
        super(props);
    }

    render() {
        return (
            <div>
                <CytoscapeComponent
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
