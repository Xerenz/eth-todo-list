import React, { Component } from 'react';
import Web3 from 'web3';
import './App.css';

class App extends Component {

  async loadBlockChainData() {
    if (window.ethereum) {
      let web3 = new Web3(window.ethereum); // new metamask

      try {
        await window.ethereum.enable(); // wait for user permission

        let accounts = await web3.eth.getAccounts();
        this.setState({account : accounts[0]});
      } catch {
        console.log("User denied account access!"); // handle condition
      }
    } 
    else if (window.web3) {
      let web3 = new Web3(web3.givenProvider || "http://localhost:8545"); // old metamask

      let accounts = await web3.eth.getAccounts();
      this.setState({account : accounts[0]});
    } 
    else {
      console.log("No provider was found");
    }
  }
  
  componentDidMount() {
    this.loadBlockChainData();
  }

  constructor(props) {
    super(props);
    this.state = {account : ''};
  }

  render() {
    return (
      <div className="container">
        <h1>Hello World</h1>
    <h3>Your Account : {this.state.account}</h3>
      </div>
    );
  }
}

export default App;
