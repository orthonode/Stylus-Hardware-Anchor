import os
from pathlib import Path
from web3 import Web3
from eth_account import Account
from dotenv import load_dotenv

load_dotenv(dotenv_path=Path(__file__).resolve().parents[1] / ".env")

# 1. Setup Connection
rpc_url = os.getenv("RPC_URL")
if not rpc_url:
    raise ValueError("RPC_URL not found in .env")

w3 = Web3(Web3.HTTPProvider(rpc_url))
print(f"Connected to Arbitrum: {w3.is_connected()}")

# 2. Account Configuration
private_key = os.getenv("PRIVATE_KEY")
if not private_key:
    raise ValueError("PRIVATE_KEY not found in .env")

if not private_key.startswith("0x"):
    private_key = "0x" + private_key

admin_account = Account.from_key(private_key)
contract_address = os.getenv("CONTRACT_ADDRESS")
if not contract_address:
    raise ValueError("CONTRACT_ADDRESS not found in .env")

contract_address = w3.to_checksum_address(contract_address)

# 3. Fixed ABI (Now valid JSON)
abi = [
    {
        "inputs": [{"name": "node_id", "type": "bytes32"}],
        "name": "authorizeNode",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function",
    }
]

contract = w3.eth.contract(address=contract_address, abi=abi)

# 4. Define the Virtual Node ID (Matches our previous discussion)
hw_id_hex = "79493f063a9316d8a39e80e66699a0937c802f54032d80d28591873130d2222d"
node_id = Web3.to_bytes(hexstr=hw_id_hex)

print(f"Authorizing Virtual Node ID: {node_id.hex()}")

# 5. Build, Sign, and Send Transaction
tx = contract.functions.authorizeNode(node_id).build_transaction(
    {
        "from": admin_account.address,
        "nonce": w3.eth.get_transaction_count(admin_account.address),
        "gas": 2000000,
        "maxFeePerGas": w3.to_wei(0.1, "gwei"),
        "maxPriorityFeePerGas": w3.to_wei(0.01, "gwei"),
        "chainId": 421614,
    }
)

signed_tx = admin_account.sign_transaction(tx)
tx_hash = w3.eth.send_raw_transaction(signed_tx.raw_transaction)

print(f"🚀 Transaction Sent! Hash: {tx_hash.hex()}")
print("Waiting for confirmation on Arbitrum Sepolia...")
receipt = w3.eth.wait_for_transaction_receipt(tx_hash)

if receipt["status"] == 1:
    print(f"✅ Confirmed in block: {receipt['blockNumber']}")
    print("Orthonode successfully authorized on-chain.")
else:
    print("❌ Transaction Reverted. Check if the node is already authorized.")
