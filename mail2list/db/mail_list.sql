create database mail2list;

\c mail2list;

BEGIN;


CREATE TABLE mail_list
(
  id INT PRIMARY KEY,
  name varchar(20)  NOT NULL,
  email varchar(20),
  archive varchar(20),
  description varchar(100)
);


