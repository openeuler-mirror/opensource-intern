create global temporary table t_gtt_trunc_dml(aa int primary key, bb int, cc text);
create index idx1_t_gtt_trunc_dml on t_gtt_trunc_dml(bb);
create table t_gtt_trunc_dml_result(id varchar(100), mtime timestamp default current_timestamp);

create global temporary table t_gtt_trunc_dml_f(aa int primary key, bb int);
alter table t_gtt_trunc_dml add foreign key(bb) references t_gtt_trunc_dml_f(aa);

create global temporary table t_gtt_trunc_ddl(aa int, bb int, cc int);
create table t_gtt_trunc_ddl_result(id varchar(100), mtime timestamp default current_timestamp);