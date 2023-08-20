-- Your SQL goes here
CREATE TABLE tasks (
    id SERIAL UNIQUE PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    state VARCHAR(20) NOT NULL,
    importance VARCHAR(20) NOT NULL,
    lifesphere VARCHAR(20) NOT NULL
)