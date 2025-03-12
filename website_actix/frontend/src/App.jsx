import { useState } from 'react';
import axios from 'axios';
import './App.css';
import { ToastContainer, toast } from 'react-toastify'; // Import Toastify
import 'react-toastify/dist/ReactToastify.css';

function App() {
  const [activeSection, setActiveSection] = useState('generate-wallet');
  const [wallets, setWallets] = useState([]);
  const [balances, setBalances] = useState([]);
  const [transaction, setTransaction] = useState(null);
  const [balanceAddress, setBalanceAddress] = useState('');
  const [senderPrivateKey, setSenderPrivateKey] = useState('');
  const [receiverAddress, setReceiverAddress] = useState('');
  const [showPopup, setShowPopup] = useState(false);

  const generateWallet = async () => {
    try {
      const response = await axios.get('http://127.0.0.1:8080/generate-wallet');
      setWallets((prevWallets) => [...prevWallets, response.data]);
      toast.success('Wallet generated successfully!');
    } catch (error) {
      console.error('Error generating wallet:', error);
      alert('Failed to generate wallet: ' + error.message); 
    }
  };

  const fetchBalance = async () => {
    if (!balanceAddress) {
      toast.warn('Please Enter an Address');
      return;
    }
    const trimmedAddress = balanceAddress.trim();
    if (!trimmedAddress) {
      toast.warn('Please enter a valid address');
      return;
    }
    try {
      const response = await axios.get(`http://127.0.0.1:8080/get-balance?address=${trimmedAddress}`);
      setBalances((prevBalances) => [...prevBalances, response.data]); // Append new balance to array
      toast.success('Balance fetched successfully!');
    } catch (error) {
      console.error('Error fetching balance:', error);
      toast.error('Failed to fetch balance: ' + error.message);
    }
  };

  const sendTransaction = async () => {
    if (!senderPrivateKey || !receiverAddress) {
      toast.warn('Please enter both sender private key and receiver address');
      return;
    }
    try {
      const response = await axios.post('http://127.0.0.1:8080/send-transaction', {
        sender_private_key: senderPrivateKey,
        receiver_address: receiverAddress,
      });
      setTransaction(response.data);
      setShowPopup(true);
    } catch (error) {
      console.error('Error sending transaction:', error);
      toast.warn('Failed to send transaction');
    }
  };

  const printReceipt = () => {
    if (!transaction || !transaction.receipt) return;
    const receipt = transaction.receipt;
    const printWindow = window.open('', '_blank');
    printWindow.document.write(`
      <html>
        <head><title>Transaction Receipt</title></head>
        <body>
          <h1>Transaction Receipt</h1>
          <p>Transaction Index: ${receipt.transaction_index}</p>
          <p>Transaction Hash: ${receipt.transaction_hash}</p>
          <p>Block Number: ${receipt.block_number}</p>
          <p>From: ${receipt.from}</p>
          <p>To: ${receipt.to}</p>
          <p>Gas Used: ${receipt.gas_used}</p>
          <p>Status: ${receipt.status}</p>
        </body>
      </html>
    `);
    printWindow.document.close();
    printWindow.print();
  };

  return (
    <div className="App">
      <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet"></link>
      <nav className="navbar">
      <div className="navbar-logo">
          <h1>ETH-TX</h1>
        </div>
        <div className="navbar-links">
        <button onClick={() => setActiveSection('generate-wallet')}>Generate Wallet</button>
        <button onClick={() => setActiveSection('get-balance')}> Get Balance</button>
        <button onClick={() => setActiveSection('send-transaction')}>Send Transaction</button>
        </div>
      </nav>

      <main>
        {activeSection === 'generate-wallet' && (
          <section>
          <h2>Generate New Wallet</h2>
          <button onClick={generateWallet}>Generate Wallet</button>
          {wallets.length > 0 && (
            <div className="result">
              <h3>Generated Wallets</h3>
              {wallets.map((wallet, index) => (
                <div key={index} className="wallet-item">
                  <p>Wallet Number: {index + 1}</p>
                  <p>Address: {wallet.address}</p>
                  <p>Private Key: {wallet.private_key}</p>
                  <p>Balance: {wallet.balance} ETH</p>
                </div>
              ))}
            </div>
          )}
        </section>
        )}

        {activeSection === 'get-balance' && (
          <section>
            <h2>Get Balance</h2>
            <input
              type="text"
              value={balanceAddress}
              onChange={(e) => setBalanceAddress(e.target.value)}
              placeholder="Enter Ethereum address"
            />
            <br></br>
            <button onClick={fetchBalance}>Check Balance</button>
            {balances.length > 0 && (
            <div className="result">
            <h3>Checked Balances</h3>
            {balances.map((balance, index) => (
              <div key={index} className="balance-item">
                <p>Number: {index + 1}</p>
                <p>Address: {balance.address}</p>
                <p>Balance: {balance.balance} ETH</p>
              </div>
            ))}
          </div>
        )}
          </section>
        )}

        {activeSection === 'send-transaction' && (
          <section>
            <h2>Send Transaction</h2>
            <input
              type="text"
              value={senderPrivateKey}
              onChange={(e) => setSenderPrivateKey(e.target.value)}
              placeholder="Sender Private Key"
            />
            <br></br>
            <input
              type="text"
              value={receiverAddress}
              onChange={(e) => setReceiverAddress(e.target.value)}
              placeholder="Receiver Address"
            />
            <br></br>
            <button onClick={sendTransaction}>Send Transaction</button>
            {transaction && (
              <div className="result">
                <p>Transaction Hash: {transaction.tx_hash}</p>
                <button onClick={printReceipt}>Print Receipt</button>
              </div>
            )}
          </section>
        )}
      </main>

      {showPopup && transaction && (
        <div className="popup">
          <div className="popup-content">
            <h3>Transaction Successful!</h3>
            <p>Transaction Hash: {transaction.tx_hash}</p>
            <button onClick={() => setShowPopup(false)}>Close</button>
          </div>
        </div>
      )}

      <ToastContainer
        position="top-right"
        autoClose={1000}
        hideProgressBar={false}
        newestOnTop={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        theme="dark"
      />
    </div>
  );
}

export default App;