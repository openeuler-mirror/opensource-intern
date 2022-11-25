create table IF NOT EXISTS mdm_meta(
fid bigint,
mode int,
uid int,
gid int,
type varchar(32),
links int,
size bigint,
parent_fid bigint, 
name varchar(256),
crtime timestamp,
atime timestamp,
mtime timestamp,
ctime timestamp,
last_rec_time timestamp,
io_time_sum bigint,
io_ratio_sum bigint,
io_time_list varchar(512),
io_ratio_list varchar(512),
nlun_list  text,
namesearch_index_col tsvector);

create table IF NOT EXISTS mdm_path(
fid bigint, 
path text,
pathsearch_index_col tsvector,
path_fid text);

CREATE VIEW infoExplorer AS SELECT
a.name,b.path,a.mode,a.uid,a.gid,a.type,a.links,a.size,a.crtime,a.atime,a.mtime,a.ctime,a.namesearch_index_col,b.pathsearch_index_col,a.last_rec_time,a.io_time_list,a.io_ratio_list,a.nlun_list FROM mdm_meta a, mdm_path b WHERE a.parent_fid=b.fid;

select * from infoexplorer where name='mdm';



create schema test_analyze_schema;
set search_path to test_analyze_schema;
create table t1_analyze_schema(a int, b int);
create user testdb_new password 'huawei@124';
alter table t1_analyze_schema owner to testdb_new;
analyze t1_analyze_schema;
drop schema test_analyze_schema cascade;

CREATE tablespace view_tablespace1  relative location 'view_tablespace1';
CREATE USER view_test1 PASSWORD 'Bigdata@123';
reset search_path;
CREATE TABLE view_t1 (a integer, b integer) ;
SET default_tablespace = view_tablespace1;
CREATE VIEW view_test1.view_v1 AS SELECT * FROM public.view_t1;
drop tablespace view_tablespace1;
drop user view_test1 cascade;
