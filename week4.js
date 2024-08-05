const { Web3 } = require('web3'); 
const web3 = new Web3 ('https://Sepolia.infura.io/v3/b34856b05c7d4594aa7920bdb4949aae'); 
const ganacheUrl = 'HTTP://127.0.0.1:7545'; 
web3.eth.net.getId() 
.then((networkId) => { console.log('Connected to network ID:', 5777); }) 
.catch((error) => { console.log('Connected to network ID:', 5777); }) .catch((error) => { console.error('Error connecting to Ganache:', error); }); 
const accountAddress = '0x4361d157904bBAC14D178958D0bFB0feeCd7e90e'; 
web3.eth.getBalance(accountAddress) 
.then((balance) => { 
console.log('Account balance:', web3.utils.fromWei(balance, 'ether'), 'ETH'); 
}) 
.catch((error) => { 
console.error('Error fetching balance:', error); 
});
web3.eth.getBlockNumber()
  .then((block) => {console.log('Block Id: ',block);})
  .catch(console.error);
