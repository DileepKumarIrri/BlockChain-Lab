const {Web3} = require('web3');  // Import Web3 library
const web3 = new Web3('https://Sepolia.infura.io/v3/b34856b05c7d4594aa7920bdb4949aae');  // Connect to Infura Sepolia
const ganacheUrl = 'HTTP://127.0.0.1:7545';  // Local Ganache URL (unused in this code)

// Fetch Network ID
web3.eth.net.getId()
  .then((networkId) => {
    console.log('Connected to network ID:', networkId);  // Correct network ID will be displayed
  })
  .catch((error) => {
    console.error('Error connecting to network:', error);  // Error handling
  });

// Fetch Account Balance
const accountAddress = '0x4361d157904bBAC14D178958D0bFB0feeCd7e90e';  // Ethereum account address
web3.eth.getBalance(accountAddress)
  .then((balance) => {
    console.log('Account balance:', web3.utils.fromWei(balance, 'ether'), 'ETH');  // Balance in Ether
  })
  .catch((error) => {
    console.error('Error fetching balance:', error);  // Error handling
  });

// Fetch Latest Block Number
web3.eth.getBlockNumber()
  .then((block) => {
    console.log('Block Id:', block);  // Latest block number
  })
  .catch(console.error);  // Error handling
