const{Web3} = require('web3');
const web3 = new Web3('https://mainnet.infura.io/v3/b34856b05c7d4594aa7920bdb4949aae');
web3.eth.getBlockNumber()
.then(console.log)
.catch(console.error);
