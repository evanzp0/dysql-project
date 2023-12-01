CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
DROP TABLE IF EXISTS test_user_2;
CREATE TABLE test_user_2 (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NULL,
    age INT NULL,
    create_at TIMESTAMPTZ NULL,
    update_at TIMESTAMP NULL
);

INSERT INTO test_user_2 (name, age, create_at, update_at) VALUES ('a1', 10, '2023-12-01 00:00:00+08', '2023-12-01 00:00:00');

DROP TABLE IF EXISTS test_user;
CREATE TABLE test_user (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NULL,
    age INT NULL
);

INSERT INTO test_user (name, age) VALUES ('huanglan', 10);
INSERT INTO test_user (name, age) VALUES ('zhanglan', 21);
INSERT INTO test_user (name, age) VALUES ('zhangsan', 35);
INSERT INTO test_user (name, age) VALUES ('a4',	12);
INSERT INTO test_user (name, age) VALUES ('a5',	21);
INSERT INTO test_user (name, age) VALUES ('a6',	22);
INSERT INTO test_user (name, age) VALUES ('a7',	24);
INSERT INTO test_user (name, age) VALUES ('a8',	31);
INSERT INTO test_user (name, age) VALUES ('a9',	33);