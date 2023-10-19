CREATE TABLE IF NOT EXISTS Contracts  (
    address FixedString(40),
    name Nullable(String),
    symbol Nullable(String),
    decimals Nullable(UInt64)
)
ENGINE = MergeTree()
ORDER BY (address)
