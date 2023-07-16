# ETHEREUM RUST INDEXER API

* Calculate network and validator participation rate (asynchronously)
* Watch new epoch events and execute code upon events.
* Save epoch ,network and validator particpation rate in MongoDB.
* Uses concurrent parallel programming

## WHY MongoDB
* Horizantal Scalability
* Ideal for real-time data
* Schema-less
## Install Prerequisites

### Rust
```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
### MongoDB
```
sudo apt-get install gnupg curl
```

```
sudo apt-get update
```


```
sudo apt-get install -y mongodb-org
```
# HOW TO RUN
- clone the repo `https://github.com/abhijeet0401/eth-rust-indexr.git`
- cd `eth-rust-indexr.git`
- start a `mongod` deamon locally (`mongodb://localhost:27017`)
- run `cargo build --release`
- run `./target/release/rust-beacon`
- wait till syncing is completed
- once completed `GET /get` endpoint will be exposed.

# HOW TO TEST

`curl http://localhost:3000/get`

`{epoch:xxxx,network participation:xxxx, validator participation:xxxx}` should be returned.

