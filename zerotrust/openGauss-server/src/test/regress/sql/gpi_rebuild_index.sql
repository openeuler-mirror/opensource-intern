--
---- test reindex global index
--

--drop table and index
drop index if exists global_partition_reindex_table_ind1;
drop index if exists global_partition_reindex_table_ind2;
drop index if exists global_partition_reindex_table_ind3;
drop index if exists partition_reindex_table_ind1;
drop index if exists partition_reindex_table_ind2;
drop table if exists partition_reindex_table;

drop index if exists partition_reindex_internal_table_ind1;
drop index if exists partition_reindex_internal_table_ind2;
drop index if exists global_partition_reindex_internal_table_ind1;
drop table if exists partition_reindex_internal_table;

--prepare table and index
create table partition_reindex_table
(
    c1 int,
    c2 int,
    c3 int
)
partition by range (c1)
(
    partition p0_partition_reindex_table values less than (10000),
    partition p1_partition_reindex_table values less than (20000),
    partition p2_partition_reindex_table values less than (30000),
    partition p3_partition_reindex_table values less than (40000),
    partition p4_partition_reindex_table values less than (50000),
    partition p5_partition_reindex_table values less than (MAXVALUE)
);
--succeed

create index partition_reindex_table_ind1 on partition_reindex_table(c1) local;
--succeed

insert into partition_reindex_table values (generate_series(1,40000), generate_series(1,40000), generate_series(1,40000));
--succeed

explain (costs off) select * from partition_reindex_table where c1 >= 19998 and c1 <= 20002 order by 1;

select * from partition_reindex_table where c1 >= 19998 and c1 <= 20002 order by 1;

create unique index global_partition_reindex_table_ind1 on partition_reindex_table(c1) global;

drop index partition_reindex_table_ind1;

create index partition_reindex_table_ind1 on partition_reindex_table(c2) local;
--succeed

create index partition_reindex_table_ind2 on partition_reindex_table(c2) local (
    PARTITION p0_partition_reindex_table,
    PARTITION p1_partition_reindex_table,
    PARTITION p2_partition_reindex_table,
    PARTITION p3_partition_reindex_table,
    PARTITION p4_partition_reindex_table,
    PARTITION p5_partition_reindex_table
);
--succeed

create unique index global_partition_reindex_table_ind1 on partition_reindex_table(c1) global;

explain (costs off) select * from partition_reindex_table where c1 >= 19998 and c1 <= 20002 order by 1;

select * from partition_reindex_table where c1 >= 19998 and c1 <= 20002 order by 1;

select c.relname,c.relpages > 0 as relpagesgtzero, c.reltuples > 0 as reltuplesgtzero,i.indisunique, i.indisvalid, i.indcheckxmin, i.indisready from pg_index i, pg_class c where c.relname = 'global_partition_reindex_table_ind1' and c.oid = i.indexrelid;

select class.relname, class.reltuples, class.parttype from pg_class class, pg_index ind where class.relname = 'global_partition_reindex_table_ind1' and ind.indexrelid = class.oid;

reindex index global_partition_reindex_table_ind1;
--succeed

explain (costs off) select * from partition_reindex_table where c1 >= 19998 and c1 <= 20002 order by 1;
--the plan before reindex and after reindex should be same

select * from partition_reindex_table where c1 >= 19998 and c1 <= 20002 order by 1;

select c.relname,c.relpages > 0 as relpagesgtzero, c.reltuples > 0 as reltuplesgtzero,i.indisunique, i.indisvalid, i.indcheckxmin, i.indisready from pg_index i, pg_class c where c.relname = 'global_partition_reindex_table_ind1' and c.oid = i.indexrelid;

select class.relname, class.reltuples, class.parttype from pg_class class, pg_index ind where class.relname = 'global_partition_reindex_table_ind1' and ind.indexrelid = class.oid;

alter index global_partition_reindex_table_ind1 rebuild;
--succeed

explain (costs off) select * from partition_reindex_table where c1 >= 19998 and c1 <= 20002 order by 1;
--the plan before reindex and after reindex should be same

select * from partition_reindex_table where c1 >= 19998 and c1 <= 20002 order by 1;

select c.relname,c.relpages > 0 as relpagesgtzero, c.reltuples > 0 as reltuplesgtzero,i.indisunique, i.indisvalid, i.indcheckxmin, i.indisready from pg_index i, pg_class c where c.relname = 'global_partition_reindex_table_ind1' and c.oid = i.indexrelid;

select class.relname, class.reltuples, class.parttype from pg_class class, pg_index ind where class.relname = 'global_partition_reindex_table_ind1' and ind.indexrelid = class.oid;

reindex table partition_reindex_table;
--succeed

explain (costs off) select * from partition_reindex_table where c1 >= 19998 and c1 <= 20002 order by 1;
--the plan before reindex and after reindex should be same

select * from partition_reindex_table where c1 >= 19998 and c1 <= 20002 order by 1;

select c.relname,c.relpages > 0 as relpagesgtzero, c.reltuples > 0 as reltuplesgtzero,i.indisunique, i.indisvalid, i.indcheckxmin, i.indisready from pg_index i, pg_class c where c.relname = 'global_partition_reindex_table_ind1' and c.oid = i.indexrelid;

select class.relname, class.reltuples, class.parttype from pg_class class, pg_index ind where class.relname = 'global_partition_reindex_table_ind1' and ind.indexrelid = class.oid;

drop index global_partition_reindex_table_ind1;
--succeed

reindex index global_partition_reindex_table_ind1;

alter index global_partition_reindex_table_ind1 rebuild;

reindex index global_partition_reindex_table_ind1 partition p0_partition_reindex_table;

alter index global_partition_reindex_table_ind1 rebuild partition p1_partition_reindex_table;

create unique index global_partition_reindex_table_ind1 on partition_reindex_table(c1) global;
--succeed

reindex index global_partition_reindex_table_ind1 partition p0_partition_reindex_table;

alter index global_partition_reindex_table_ind1 rebuild partition p1_partition_reindex_table;

create index global_partition_reindex_table_ind1 on partition_reindex_table using btree(c1) global;

create index global_partition_reindex_table_ind2 on partition_reindex_table using btree(c1) global;
--succeed

create index global_partition_reindex_table_ind3 on partition_reindex_table using btree(c1) global;
--succeed

reindex index global_partition_reindex_table_ind1;
reindex index global_partition_reindex_table_ind1 partition p0_partition_reindex_table;
reindex table partition_reindex_table;
reindex internal table partition_reindex_table;
reindex table partition_reindex_table partition p0_partition_reindex_table;
reindex internal table partition_reindex_table partition p0_partition_reindex_table;
alter index global_partition_reindex_table_ind1 rebuild;

create table partition_reindex_internal_table(
    c_id    varchar,
    c_w_id  integer,
    c_date  date,
    partial cluster key(c_id,c_w_id)
) with (orientation = column, max_batchrow = 30700, compression = middle)
partition by range (c_date, c_w_id)
(
    PARTITION p0_partition_reindex_internal_table values less than ('20170331',5),
    PARTITION p1_partition_reindex_internal_table values less than ('20170731',450),
    PARTITION p2_partition_reindex_internal_table values less than ('20170930',1062),
    PARTITION p3_partition_reindex_internal_table values less than ('20171231',1765),
    PARTITION p4_partition_reindex_internal_table values less than ('20180331',2024),
    PARTITION p5_partition_reindex_internal_table values less than ('20180731',2384),
    PARTITION p6_partition_reindex_internal_table values less than ('20180930',2786),
    PARTITION p7_partition_reindex_internal_table values less than (maxvalue,maxvalue)
);

insert into partition_reindex_internal_table values('gauss1',4,'20170301');
insert into partition_reindex_internal_table values('gauss2',400,'20170625');
insert into partition_reindex_internal_table values('gauss3',480,'20170920');
insert into partition_reindex_internal_table values('gauss4',1065,'20170920');
insert into partition_reindex_internal_table values('gauss5',1800,'20170920');
insert into partition_reindex_internal_table values('gauss6',2030,'20170920');
insert into partition_reindex_internal_table values('gauss7',2385,'20170920');
insert into partition_reindex_internal_table values('gauss8',2789,'20191020');
insert into partition_reindex_internal_table values('gauss9',2789,'20171020');

create index partition_reindex_internal_table_ind1 on partition_reindex_internal_table using btree(c_w_id) LOCAL;
create index partition_reindex_internal_table_ind2 on partition_reindex_internal_table using btree(c_w_id) LOCAL (
    PARTITION p0_partition_reindex_internal_table,
    PARTITION p1_partition_reindex_internal_table,
    PARTITION p2_partition_reindex_internal_table,
    PARTITION p3_partition_reindex_internal_table,
    PARTITION p4_partition_reindex_internal_table,
    PARTITION p5_partition_reindex_internal_table,
    PARTITION p6_partition_reindex_internal_table,
    PARTITION p7_partition_reindex_internal_table
);

create index global_partition_reindex_internal_table_ind1 on partition_reindex_internal_table using btree(c_id) global;

reindex index global_partition_reindex_internal_table_ind1;
reindex index global_partition_reindex_internal_table_ind1 partition p0_partition_reindex_internal_table;
reindex table partition_reindex_internal_table;
reindex internal table partition_reindex_internal_table;
reindex table partition_reindex_internal_table partition p0_partition_reindex_internal_table;
reindex internal table partition_reindex_internal_table partition p0_partition_reindex_internal_table;
alter index global_partition_reindex_internal_table_ind1 rebuild;

\parallel on
reindex index global_partition_reindex_table_ind1;
reindex index global_partition_reindex_table_ind1;
reindex index global_partition_reindex_table_ind1;
reindex index global_partition_reindex_table_ind1;
reindex index global_partition_reindex_table_ind1;
reindex index global_partition_reindex_table_ind1;
\parallel off

\parallel on
reindex table partition_reindex_table;
reindex table partition_reindex_table;
reindex table partition_reindex_table;
reindex table partition_reindex_table;
reindex table partition_reindex_table;
reindex table partition_reindex_table;
\parallel off

\parallel on
reindex internal table partition_reindex_internal_table;
reindex internal table partition_reindex_internal_table;
reindex internal table partition_reindex_internal_table;
reindex internal table partition_reindex_internal_table;
reindex internal table partition_reindex_internal_table;
reindex internal table partition_reindex_internal_table;
\parallel off

\parallel on
alter index global_partition_reindex_table_ind1 rebuild;
alter index global_partition_reindex_table_ind1 rebuild;
alter index global_partition_reindex_table_ind1 rebuild;
alter index global_partition_reindex_table_ind1 rebuild;
alter index global_partition_reindex_table_ind1 rebuild;
alter index global_partition_reindex_table_ind1 rebuild;
\parallel off

\parallel on
alter index global_partition_reindex_table_ind1 rebuild;
reindex table partition_reindex_table;
alter index global_partition_reindex_table_ind1 rebuild;
alter index global_partition_reindex_table_ind1 rebuild;
reindex table partition_reindex_table;
alter index global_partition_reindex_table_ind1 rebuild;
\parallel off

\parallel on
reindex index global_partition_reindex_table_ind1;
reindex table partition_reindex_table;
alter index global_partition_reindex_table_ind1 rebuild;
reindex index global_partition_reindex_table_ind1;
reindex table partition_reindex_table;
alter index global_partition_reindex_table_ind1 rebuild;
\parallel off

\parallel on
reindex index global_partition_reindex_table_ind1;
reindex index global_partition_reindex_table_ind1;
reindex index global_partition_reindex_table_ind1;
reindex index global_partition_reindex_table_ind2;
reindex index global_partition_reindex_table_ind1;
reindex index global_partition_reindex_table_ind3;
\parallel off

\parallel on
reindex index global_partition_reindex_table_ind1;
reindex index global_partition_reindex_table_ind2;
reindex index global_partition_reindex_table_ind3;
reindex table partition_reindex_table;
reindex table partition_reindex_table;
reindex table partition_reindex_table;
\parallel off

--clean
drop index if exists global_partition_reindex_table_ind1;
drop index if exists global_partition_reindex_table_ind2;
drop index if exists global_partition_reindex_table_ind3;
drop index if exists partition_reindex_table_ind1;
drop index if exists partition_reindex_table_ind2;
drop table if exists partition_reindex_table;

drop index if exists partition_reindex_internal_table_ind1;
drop index if exists partition_reindex_internal_table_ind2;
drop index if exists global_partition_reindex_internal_table_ind1;
drop table if exists partition_reindex_internal_table;
