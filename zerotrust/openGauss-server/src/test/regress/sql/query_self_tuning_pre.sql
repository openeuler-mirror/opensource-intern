\c postgres
create schema sql_self_tuning;
set current_schema='sql_self_tuning';

create node group sql_self_tuning_01 with (datanode1, datanode3, datanode5,datanode7);
create node group sql_self_tuning_02 with (datanode2, datanode4, datanode6);

create table t1 ( c1 integer[], c2 int) with(autovacuum_enabled = off) distribute by replication;
create table t2 (c1 int, c2 hstore) with(autovacuum_enabled = off);
create table t3(c1 text, c2 text, c3 int) with(autovacuum_enabled = off);

create table t5(c1 int, c2 int, c3 int, c4 int) with(autovacuum_enabled = off) distribute by hash(c4);
insert into t5 select v % 5,v,v, 0 from generate_series(1,1024) as v;
create table t4 with(autovacuum_enabled = off) as select * from t5;
insert into t5 select * from t5;
insert into t5 select * from t5;
insert into t5 select * from t5;
insert into t5 select * from t5;
insert into t5 select * from t5;
insert into t5 select * from t5;
insert into t5 select * from t5;
insert into t5 select * from t5;
insert into t5 select * from t5;
create table t13 with(autovacuum_enabled = off) as select * from t5;
insert into t13 select * from t13;
insert into t13 select * from t13;
create table t14(c1 int, c2 int, c3 int, c4 int) with(autovacuum_enabled = off) distribute by hash(c2) to group sql_self_tuning_01;
create table t15(c1 int, c2 int, c3 int, c4 int) with(autovacuum_enabled = off) distribute by hash(c4) to group sql_self_tuning_02;
insert into t14 select * from t5;
insert into t15 select * from t5;
analyze t5;
analyze t13;
analyze t14;
analyze t15;

create table t6(c1 bytea, c2 bytea,c3 int) with(autovacuum_enabled = off);

create table t16(c1 int, c2 int, c3 int, c4 int) with(autovacuum_enabled = off) distribute by hash(c1) to group sql_self_tuning_02;
insert into t16 select * from t5;
create index idx_t16 on t16(c4);
create table ct16(c1 int, c2 int, c3 int, c4 int) with(autovacuum_enabled = off, ORIENTATION = column) distribute by hash(c2) to group sql_self_tuning_02;
insert into ct16 select * from t5;
create index idx_ct16 on ct16(c1);

\c regression
