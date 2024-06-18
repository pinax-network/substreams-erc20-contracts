CREATE TABLE IF NOT EXISTS contracts  (
    contract FixedString(40),
    name        String,
    symbol      String,
    decimals    UInt64,
    block_num   UInt32(),
    timestamp   DateTime64(3, 'UTC'),
)
ENGINE = MergeTree PRIMARY KEY ("contract")
ORDER BY (contract, name);


