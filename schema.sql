CREATE TABLE IF NOT EXISTS Contracts  (
    chain           LowCardinality(String),
    block_number    UInt32(),
    timestamp       DateTime64(3, 'UTC'),
    address FixedString(40),
    name Nullable(String),
    symbol Nullable(String),
    decimals Nullable(UInt64)
)
ENGINE = MergeTree()
ORDER BY (address)
