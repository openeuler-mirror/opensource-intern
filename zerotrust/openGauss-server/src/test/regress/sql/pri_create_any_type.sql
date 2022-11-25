CREATE USER test_create_any_type_role PASSWORD 'Gauss@1234';
GRANT create any type to test_create_any_type_role;

CREATE SCHEMA pri_type_schema;
set search_path=pri_type_schema;

SET ROLE test_create_any_type_role PASSWORD 'Gauss@1234';

CREATE TYPE pri_type_schema.compfoo AS (f1 int, f2 text);
CREATE TABLE test_create_any_type_role.t1_compfoo(a int, b pri_type_schema.compfoo);

--创建一个枚举类型
CREATE TYPE pri_type_schema.bugstatus AS ENUM ('create', 'modify', 'closed');
create type pri_type_schema.textrange_c as range(subtype=text, collation="C");

CREATE TYPE pri_type_schema.bigobj (INPUT = lo_filein, OUTPUT = lo_fileout, INTERNALLENGTH = VARIABLE);

CREATE TYPE pri_type_schema.int42;
CREATE TYPE pri_type_schema.text_w_default;
reset role;
--校验type的drop权限
CREATE USER test_create_any_type_role_test PASSWORD 'Gauss@1234';
GRANT create any type to test_create_any_type_role_test;
SET ROLE test_create_any_type_role_test PASSWORD 'Gauss@1234';
DROP TYPE pri_type_schema.compfoo;
reset role;

CREATE FUNCTION pri_type_schema.int42_in(cstring)
   RETURNS pri_type_schema.int42
   AS 'int4in'
   LANGUAGE internal STRICT;
CREATE FUNCTION pri_type_schema.int42_out(pri_type_schema.int42)
   RETURNS cstring
   AS 'int4out'
   LANGUAGE internal STRICT;

CREATE FUNCTION pri_type_schema.text_w_default_in(cstring)
   RETURNS pri_type_schema.text_w_default
   AS 'textin'
   LANGUAGE internal STRICT;

CREATE FUNCTION pri_type_schema.text_w_default_out(pri_type_schema.text_w_default)
   RETURNS cstring
   AS 'textout'
   LANGUAGE internal STRICT;
SET ROLE test_create_any_type_role PASSWORD 'Gauss@1234';
CREATE TYPE pri_type_schema.int42 (
   internallength = 4,
   input = pri_type_schema.int42_in,
   output = pri_type_schema.int42_out,
   alignment = int4,
   default = 42,
   passedbyvalue
);
CREATE TYPE pri_type_schema.text_w_default (
   internallength = variable,
   input = pri_type_schema.text_w_default_in,
   output = pri_type_schema.text_w_default_out,
   alignment = int4,
   default = 'zippo'
);
---failed
CREATE TABLE pri_type_schema.default_test (f1 int, f2 int);
CREATE SEQUENCE  pri_type_schema.sequence_test1 START WITH 32;
CREATE FUNCTION pri_type_schema.pri_func_add_sql(integer, integer) RETURNS integer
AS 'select $1 + $2;'
LANGUAGE SQL
IMMUTABLE
RETURNS NULL ON NULL INPUT;

reset role;
DROP TABLE test_create_any_type_role.t1_compfoo;
drop type pri_type_schema.compfoo;
drop type pri_type_schema.bugstatus;
drop function pri_type_schema.int42_in(cstring);
drop function pri_type_schema.int42_out(int42);
drop type pri_type_schema.int42;
drop function pri_type_schema.text_w_default_in(cstring);
drop function pri_type_schema.text_w_default_out(text_w_default);
drop type pri_type_schema.text_w_default;

DROP SCHEMA pri_type_schema cascade;
DROP USER test_create_any_type_role cascade;
DROP USER test_create_any_type_role_test cascade;
