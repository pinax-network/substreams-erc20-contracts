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
ORDER BY (address, timestamp, block_number, chain);

-- Indexes for block_number and chain --
ALTER TABLE Contracts ADD INDEX Contracts_block_number_index block_number TYPE minmax;
ALTER TABLE Contracts ADD INDEX Contracts_chain_index chain TYPE minmax;
