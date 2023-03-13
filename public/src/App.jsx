import React, { Component } from 'react';
import { Route, Routes } from 'react-router-dom';
import Home from './pages/Home';

class App extends Component {
    render() {
        return (
            <div>
                <Routes>
                    <Route path="/" element={<Home></Home>}></Route>
                </Routes>
            </div>
        );
    }
}

export default App;