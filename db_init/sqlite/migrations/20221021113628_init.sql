DROP TABLE IF EXISTS test_user;
CREATE TABLE test_user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NULL,
    age INT NULL
);

INSERT INTO test_user (name, age) VALUES ('huanglan', 10);
INSERT INTO test_user (name, age) VALUES ('zhanglan', 21);
INSERT INTO test_user (name, age) VALUES ('zhangsan', 35);