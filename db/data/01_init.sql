CREATE USER murasaki WITH PASSWORD 'murasaki' SUPERUSER;
CREATE DATABASE todolist owner murasaki encoding 'UTF8';
GRANT ALL PRIVILEGES ON DATABASE todolist TO murasaki;

\c todolist;
CREATE SCHEMA todolist;

SET client_encoding = 'UTF8';
