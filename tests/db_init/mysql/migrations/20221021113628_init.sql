DROP TABLE IF EXISTS test_user_2;
CREATE TABLE test_user_2 (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NULL,
    age INT NULL,
    create_at TIMESTAMP NULL,
    update_at DATETIME NULL,
    update_at2 TIMESTAMP NULL
);

INSERT INTO test_user_2 (name, age, create_at, update_at, update_at2) VALUES ('a1', 10, '2023-12-01 00:00:00', '2023-12-01 00:00:00', '2023-12-01 00:00:00');

DROP TABLE IF EXISTS test_user;
CREATE TABLE test_user (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NULL,
    age INT NULL
);

INSERT INTO test_user (name, age) VALUES ('a1', 10);
INSERT INTO test_user (name, age) VALUES ('a2', 21);
INSERT INTO test_user (name, age) VALUES ('a3', 35);
INSERT INTO test_user (name, age) VALUES ('a4',	12);
INSERT INTO test_user (name, age) VALUES ('a5',	21);
INSERT INTO test_user (name, age) VALUES ('a6',	22);
INSERT INTO test_user (name, age) VALUES ('a7',	24);
INSERT INTO test_user (name, age) VALUES ('a8',	31);
INSERT INTO test_user (name, age) VALUES ('a9',	33);

