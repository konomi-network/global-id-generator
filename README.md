## global-id-generator

* The global ID generation service is mainly used in distributed business scenarios to ensure unique IDs.
* The snowflake algorithm and MySQL self increasing ID method each have their own advantages and disadvantages. 
* To ensure the order and high performance of business IDs, Trust and Rocksdb are used to solve the problem. 
* At the same time, for the convenience of business self definition, interfaces can be inherited to implement their own ID generation logic.
* Here, two http interfaces`/id?sharding_id=xx` (generate one id at a time) and `/id?sharding_id=?&num=xx` (generating multiple IDs at once)


### 1. define config.yaml

```yaml
increment: 1
starting: 0
sharding_bits: 0
address: "0.0.0.0:8080"  # server port
channel_size: 500
rocksdb_path: ".rocksdb"  # if running on docker-compose or k8s, this path needs to be mounted
rocksdb_storage_key: [0]
```

### 2. compile and run server

```shell
cargo run
```