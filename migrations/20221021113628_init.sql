-- Add migration script here
DROP TABLE IF EXISTS test_user;
CREATE TABLE test_user (
    id INT NOT NULL PRIMARY KEY,
    name VARCHAR(255) NULL,
    age INT NULL
);

INSERT INTO test_user (id, name, age) VALUES (1, 'huanglan', 10);
INSERT INTO test_user (id, name, age) VALUES (2, 'zhanglan', 21);
INSERT INTO test_user (id, name, age) VALUES (3, 'zhangsan', 35);