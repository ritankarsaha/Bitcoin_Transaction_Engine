
set -e

echo "Starting bitcoind in regtest mode..."
bitcoind -regtest -daemon \
  -rpcuser=bitcoin -rpcpassword=password \
  -rpcallowip=0.0.0.0/0 -rpcbind=0.0.0.0

echo "Waiting for bitcoind RPC to be available..."
sleep 5

count=0
until bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=password getblockchaininfo > /dev/null 2>&1
do
  ((count++))
  if [ $count -gt 30 ]; then
    echo "RPC connection timed out"
    exit 1
  fi
  sleep 2
done
echo "bitcoind is up!"

echo "Creating legacy wallet 'default'..."
bitcoin-cli -regtest -rpcuser=bitcoin -rpcpassword=password createwallet "default" false false "" false false \
  || echo "Wallet may already exist"

echo "Running Rust demo..."
cd /app/rust-app
RUST_BACKTRACE=1 ./target/release/bitcoin-tx-demo


tail -f /dev/null
