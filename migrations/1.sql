CREATE TABLE test_table (
    id UInt32,
    name String,
    age UInt8,
    email String,
    created_at DateTime,
    is_active UInt8,
    score Float32
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(created_at)
ORDER BY id;
#
CREATE TABLE test_table2 (
    id UInt32,
    name String,
    age UInt8,
    email String,
    created_at DateTime,
    is_active UInt8,
    score Float32
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(created_at)
ORDER BY id;