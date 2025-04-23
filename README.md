# P2Poolv2 Atomic-Swap Module

A Rust-based toolkit for trustless, non-custodial atomic swaps of P2Pool v2 sharechain rewards into Bitcoin, leveraging both the Lightning Network (via LDK or Core-Lightning) and the Ark protocol.

## Steps to run the project properly

1. Clone the repository properly

```bash
git clone https://github.com/ritankarsaha/Bitcoin_Transaction_Engine.git
```
2. Move into the repository

```bash
cd Bitcoin_Transaction_Engine
```

3. Build the environment and execute sample Liquid transactions

```bash
sudo docker compose up --build
```

The app is good to run :) You can View the outputs on the terminal properly. Logs also have been added for more clarity. 


If you had already earlier taken the project for a spin, a default wallet has already been created, so the new wallet doesn't load. It will be using the previous wallet by default and you will get an output somewhat like this. :)

```bash
Compose can now delegate builds to bake for better performance.
 To do so, set COMPOSE_BAKE=true.
[+] Building 1.1s (17/17) FINISHED                                                docker:default
 => [btc-node internal] load build definition from Dockerfile                               0.0s
 => => transferring dockerfile: 1.19kB                                                      0.0s
 => [btc-node internal] load metadata for docker.io/library/ubuntu:22.04                    1.0s
 => [btc-node internal] load .dockerignore                                                  0.0s
 => => transferring context: 2B                                                             0.0s
 => [btc-node  1/11] FROM docker.io/library/ubuntu:22.04@sha256:d80997daaa3811b175119350d8  0.0s
 => [btc-node internal] load build context                                                  0.0s
 => => transferring context: 79.83kB                                                        0.0s
 => CACHED [btc-node  2/11] RUN apt-get update && apt-get install -y     wget     curl      0.0s
 => CACHED [btc-node  3/11] RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs   0.0s
 => CACHED [btc-node  4/11] RUN wget https://bitcoincore.org/bin/bitcoin-core-25.0/bitcoin  0.0s
 => CACHED [btc-node  5/11] RUN mkdir -p /root/.bitcoin /app                                0.0s
 => CACHED [btc-node  6/11] WORKDIR /app                                                    0.0s
 => CACHED [btc-node  7/11] COPY scripts/bitcoin.conf /root/.bitcoin/bitcoin.conf           0.0s
 => CACHED [btc-node  8/11] COPY rust-app /app/rust-app                                     0.0s
 => CACHED [btc-node  9/11] RUN cd /app/rust-app && cargo build --release                   0.0s
 => CACHED [btc-node 10/11] COPY scripts/start.sh /app/start.sh                             0.0s
 => CACHED [btc-node 11/11] RUN chmod +x /app/start.sh                                      0.0s
 => [btc-node] exporting to image                                                           0.0s
 => => exporting layers                                                                     0.0s
 => => writing image sha256:4394bdeed8d134546822b9891e3f7ad13358f11d68458d4f57c64e60300673  0.0s
 => => naming to docker.io/library/bitcoin-tx-demo-btc-node                                 0.0s
 => [btc-node] resolving provenance for metadata file                                       0.0s
[+] Running 3/3
 ✔ btc-node                         Built                                                   0.0s 
 ✔ Network bitcoin-tx-demo_default  Created                                                 0.0s 
 ✔ Container bitcoin-tx-demo        Created                                                 0.0s 
Attaching to bitcoin-tx-demo
bitcoin-tx-demo  | Starting bitcoind in regtest mode...
bitcoin-tx-demo  | Bitcoin Core starting
bitcoin-tx-demo  | Waiting for bitcoind RPC to be available...
bitcoin-tx-demo  | bitcoind is up!
bitcoin-tx-demo  | Creating legacy wallet 'default'...
bitcoin-tx-demo  | error code: -4
bitcoin-tx-demo  | error message:
bitcoin-tx-demo  | Wallet file verification failed. Failed to create database path '/root/.bitcoin/regtest/wallets/default'. Database already exists.
bitcoin-tx-demo  | Wallet may already exist
bitcoin-tx-demo  | Running Rust demo...
bitcoin-tx-demo  | Starting Bitcoin transaction demo...
bitcoin-tx-demo  | Waiting for Bitcoin RPC...
bitcoin-tx-demo  | Connected to Bitcoin node.
bitcoin-tx-demo  | Generated address: mugQnBxTR8z3uqzZJV36RsYR5yX1g64do1
bitcoin-tx-demo  | Importing private key...
bitcoin-tx-demo  | Mining 101 blocks...
bitcoin-tx-demo  | UTXO: aba53aba46a7d6fb67b52d6c611171dd1f9044e75f0718f8e1142186778bc4ca:0 50 BTC
bitcoin-tx-demo  | First txid: c41618bb5429b0ebb1a0d8eddbd24ccfbdbb1977b51feda2eb0b9046c22e916b
bitcoin-tx-demo  | UTXO2: c41618bb5429b0ebb1a0d8eddbd24ccfbdbb1977b51feda2eb0b9046c22e916b:0 49.99999 BTC
bitcoin-tx-demo  | Second txid: 77275e49170f6aef186e920f30a832700d2bb10d5b5f9bb6d53bd85588f18785
bitcoin-tx-demo  | Demo complete.
```

However, if you would prefer to initialize a fresh wallet and perform transactions from it, run the following commands:

```bash
sudo docker compose down -v
sudo docker compose up —build
```

You should see output similar to below, indicating that a new default wallet has been created and the transaction executed successfully:

```bash
Compose can now delegate builds to bake for better performance.
 To do so, set COMPOSE_BAKE=true.
[+] Building 1.1s (17/17) FINISHED                                                                                                                                docker:default
 => [btc-node internal] load build definition from Dockerfile                                                                                                               0.0s
 => => transferring dockerfile: 1.19kB                                                                                                                                      0.0s
 => [btc-node internal] load metadata for docker.io/library/ubuntu:22.04                                                                                                    0.9s
 => [btc-node internal] load .dockerignore                                                                                                                                  0.0s
 => => transferring context: 2B                                                                                                                                             0.0s
 => [btc-node  1/11] FROM docker.io/library/ubuntu:22.04@sha256:d80997daaa3811b175119350d84305e1ec9129e1799bba0bd1e3120da3ff52c3                                            0.0s
 => [btc-node internal] load build context                                                                                                                                  0.0s
 => => transferring context: 79.83kB                                                                                                                                        0.0s
 => CACHED [btc-node  2/11] RUN apt-get update && apt-get install -y     wget     curl     gnupg     build-essential     pkg-config     libssl-dev     ca-certificates      0.0s
 => CACHED [btc-node  3/11] RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y                                                                     0.0s
 => CACHED [btc-node  4/11] RUN wget https://bitcoincore.org/bin/bitcoin-core-25.0/bitcoin-25.0-x86_64-linux-gnu.tar.gz     && tar -xzf bitcoin-25.0-x86_64-linux-gnu.tar.  0.0s
 => CACHED [btc-node  5/11] RUN mkdir -p /root/.bitcoin /app                                                                                                                0.0s
 => CACHED [btc-node  6/11] WORKDIR /app                                                                                                                                    0.0s
 => CACHED [btc-node  7/11] COPY scripts/bitcoin.conf /root/.bitcoin/bitcoin.conf                                                                                           0.0s
 => CACHED [btc-node  8/11] COPY rust-app /app/rust-app                                                                                                                     0.0s
 => CACHED [btc-node  9/11] RUN cd /app/rust-app && cargo build --release                                                                                                   0.0s
 => CACHED [btc-node 10/11] COPY scripts/start.sh /app/start.sh                                                                                                             0.0s
 => CACHED [btc-node 11/11] RUN chmod +x /app/start.sh                                                                                                                      0.0s
 => [btc-node] exporting to image                                                                                                                                           0.0s
 => => exporting layers                                                                                                                                                     0.0s
 => => writing image sha256:4394bdeed8d134546822b9891e3f7ad13358f11d68458d4f57c64e60300673cb                                                                                0.0s
 => => naming to docker.io/library/bitcoin-tx-demo-btc-node                                                                                                                 0.0s
 => [btc-node] resolving provenance for metadata file                                                                                                                       0.0s
[+] Running 4/4
 ✔ btc-node                               Built                                                                                                                             0.0s 
 ✔ Network bitcoin-tx-demo_default        Created                                                                                                                           0.0s 
 ✔ Volume "bitcoin-tx-demo_bitcoin-data"  Created                                                                                                                           0.0s 
 ✔ Container bitcoin-tx-demo              Created                                                                                                                           0.0s 
Attaching to bitcoin-tx-demo
bitcoin-tx-demo  | Starting bitcoind in regtest mode...
bitcoin-tx-demo  | Bitcoin Core starting
bitcoin-tx-demo  | Waiting for bitcoind RPC to be available...
bitcoin-tx-demo  | bitcoind is up!
bitcoin-tx-demo  | Creating legacy wallet 'default'...
bitcoin-tx-demo  | {
bitcoin-tx-demo  |   "name": "default",
bitcoin-tx-demo  |   "warnings": [
bitcoin-tx-demo  |     "Empty string given as passphrase, wallet will not be encrypted.",
bitcoin-tx-demo  |     "Wallet created successfully. The legacy wallet type is being deprecated and support for creating and opening legacy wallets will be removed in the future."
bitcoin-tx-demo  |   ]
bitcoin-tx-demo  | }
bitcoin-tx-demo  | Running Rust demo...
bitcoin-tx-demo  | Starting Bitcoin transaction demo...
bitcoin-tx-demo  | Waiting for Bitcoin RPC...
bitcoin-tx-demo  | Connected to Bitcoin node.
bitcoin-tx-demo  | Note: load_wallet error: JSON-RPC error: RPC error response: RpcError { code: -35, message: "Wallet \"default\" is already loaded.", data: None }
bitcoin-tx-demo  | Generated address: mxYuwX52hCh2Uxq1P9M4d21w2LtBVSoVG1
bitcoin-tx-demo  | Importing private key...
bitcoin-tx-demo  | Mining 101 blocks...
bitcoin-tx-demo  | UTXO: 465262bbb4183cac2d2af6ee5d69ffde7476067bb362e471320e4f4d0da29547:0 50 BTC
bitcoin-tx-demo  | First txid: f77d1514f8dcab8db0d4de8a595ef13c15fc526304c3215d1904db0c864ee4a8
bitcoin-tx-demo  | UTXO2: f77d1514f8dcab8db0d4de8a595ef13c15fc526304c3215d1904db0c864ee4a8:0 49.99999 BTC
bitcoin-tx-demo  | Second txid: abd3fb951a0e7bdae5d970f7e72b458235eebbae806bce0909f698df854e2458
bitcoin-tx-demo  | Demo complete.
```

Now, let’s take this project for another spin and dive into some deeper testing 🙂

If you would rather create your own wallets manually from the terminal instead of using the auto-generated default one, just follow these steps below 👇

1. Start the docker container. Make sure to build it as well if you have removed containers earlier by `sudo docker compose down -v`

```bash
sudo docker compose start  
```

Output
```bash
+] Running 1/1
✔ Container bitcoin-tx-demo  Started
```
2. Generate and create the new wallets on your disk properly by running this command

```bash
docker exec bitcoin-tx-demo \
bitcoin-cli -regtest createwallet "test1"     false false "" false false

docker exec bitcoin-tx-demo \
bitcoin-cli -regtest createwallet "experiment" false false "" false false
```

Output
The output shows that two wallets have been created properly on the disk -- 

```bash
{
"name": "test1",
"warnings": [
"Empty string given as passphrase, wallet will not be encrypted.",
"Wallet created successfully. The legacy wallet type is being deprecated and support for creating and opening legacy wallets will be removed in the future."
]
}
{
"name": "experiment",
"warnings": [
"Empty string given as passphrase, wallet will not be encrypted.",
"Wallet created successfully. The legacy wallet type is being deprecated and support for creating and opening legacy wallets will be removed in the future."
]
}
```

Before running the following list of commands make sure that your container is properly up and running :)
```bash
sudo docker compose start
```

If you wish to unload your default wallet then follow this command -- 

```bash
docker compose exec btc-node \
  bitcoin-cli -regtest unloadwallet default >/dev/null 2>&1 || true
```
This command will unload your default wallet .

If you want to list all the wallets which are at present , present on your disk follow this command -- 

```bash
docker compose exec btc-node \
  bitcoin-cli -regtest listwalletdir
```

Output -- 
In the output you can see the default wallet has been unloaded successfully and you can see the remaining two wallets on the disk which we had created earlier -- 

```bash
{
"wallets": [
{
"name": "default"
},
{
"name": "test1"
},
{
"name": "experiment"
}
]
}
```

Furthermore, if you want to see the currently loaded wallets on your disk (note -  we have already unloaded the default wallet) run this following command -- 

```bash
docker compose exec btc-node \
  bitcoin-cli -regtest listwallets
```

Output -- 
The output will show you all the wallets you have currently loaded on your disk -- Note that the output will not be showing the default wallet as we have already unloaded the wallet from our disk

```bash
[
"test1",
"experiment"
]
```

Now, suppose we had unloaded the `test1` wallet from our disk, there is also a way of reloading them back :) Just follow the below commands for them -- 

```bash
docker compose exec btc-node \
  bitcoin-cli -regtest loadwallet "test1"
```

Output -
The output now shows that you have loaded the wallet properly on your disk and it is up for running. 

```bash
{
"name": "test1"
}
```


Ritankar Saha
Github - https://github.com/ritankarsaha.






