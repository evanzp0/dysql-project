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