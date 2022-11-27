--kinds of hash columns
set datestyle = 'iso, ymd';
set intervalstyle to postgres;
set time zone prc;

--I1.create table
create table redistable.redis_table_0001 distribute by hash(c_int8) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0002 distribute by hash(c_int1) as select *, cast(c_int8 as int1) as c_int1 from redistable.redis_table_0000;
create table redistable.redis_table_0003 distribute by hash(c_int2) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0004 distribute by hash(c_int4) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0005 distribute by hash(c_numeric) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0006 distribute by hash(c_char) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0007 distribute by hash(c_bpchar) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0008 distribute by hash(c_varchar) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0009 distribute by hash(c_nvarchar2) as select *, cast(c_varchar as nvarchar2) as c_nvarchar2 from redistable.redis_table_0000;
create table redistable.redis_table_0010 distribute by hash(c_date) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0011 distribute by hash(c_time) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0012 distribute by hash(c_timestamp) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0013 distribute by hash(c_timestamptz) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0014 distribute by hash(c_interval) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0015 distribute by hash(c_timetz) as select * from redistable.redis_table_0000;
create table redistable.redis_table_0016 distribute by hash(c_smalldatetime) as select *, cast(c_timestamptz as smalldatetime) as c_smalldatetime from redistable.redis_table_0000;

create table redistable.redis_table_1001 distribute by hash(c_oid) as select * from redistable.redis_table_0000;
create table redistable.redis_table_1002 distribute by hash(c_abstime) as select * from redistable.redis_table_0000;
create table redistable.redis_table_1003 distribute by hash(c_reltime) as select * from redistable.redis_table_0000;
create table redistable.redis_table_1004 distribute by hash(c_money) as select * from redistable.redis_table_0000;
create table redistable.redis_table_1005 distribute by hash(c_bytea) as select * from redistable.redis_table_0000;
create table redistable.redis_table_1006 distribute by hash(c_raw) as select *, cast(c_int8::text as raw) as c_raw from redistable.redis_table_0000;
create table redistable.redis_table_1007 distribute by hash(c_bool) as select * from redistable.redis_table_0000;
create table redistable.redis_table_1008 distribute by hash(c_name) as select * from redistable.redis_table_0000;
create table redistable.redis_table_1009 distribute by hash(c_int2vector) as select * from redistable.redis_table_0000;
create table redistable.redis_table_1010 distribute by hash(c_text) as select * from redistable.redis_table_0000;
create table redistable.redis_table_1011 distribute by hash(c_oidvector) as select * from redistable.redis_table_0000;
create table redistable.redis_table_1012 distribute by hash(c_float4) as select * from redistable.redis_table_0000;
create table redistable.redis_table_1013 distribute by hash(c_float8) as select * from redistable.redis_table_0000;

CREATE TABLE Cust_h_omer (
c_id int ,
c_d_id int ,
c_w_id int ,
c_first varchar(16) ,
c_middle char(2) ,
c_last varchar(16) ,
c_street_1 varchar(20) ,
c_street_2 varchar(20) ,
c_city varchar(20) ,
c_state char(2) ,
c_zip char(9) ,
c_phone char(16) ,
c_since timestamp ,
c_credit char(2) ,
c_credit_lim numeric(12,2) ,
c_discount numeric(4,4) ,
c_balance numeric(12,2) ,
c_ytd_payment numeric(12,2) ,
c_payment_cnt int ,
c_delivery_cnt int ,
c_data varchar(500), partial cluster key(c_since))
distribute by hash(c_w_id);
--I2.select to verfify
