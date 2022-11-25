/* Test 1 */
create table t1 (c1 int) with (storage_type=ustore);
insert into t1 values (0);

\parallel on 3

declare
    c int;
begin
    pg_sleep(1);
    select c1 into c from t1 for share;
    pg_sleep(1);
    raise notice 'txn 1 for share %', c;
    update t1 set c1 = c1 * 2; -- wait multixid(trxn 1 and trxn 2) and update
end;
/

begin
    perform * from t1 for update;
    pg_sleep(1.5);
    raise notice 'txn 2 for update';
    update t1 set c1 = c1 + 1;
end;
/

declare
    c int;
begin
    pg_sleep(1);
    select c1 into c from t1 for share;
    pg_sleep(0.5);
    raise notice 'txn 3 for share %', c;
    pg_sleep(2);
end;
/

\parallel off

select * from t1; /* 2 */
drop table t1;



/* Test 2 */
create table t1 (c1 int) with (storage_type=ustore);
insert into t1 values (0);

\parallel on 3

declare
    c int;
begin
    pg_sleep(1);
    select c1 into c from t1 for share;
    pg_sleep(1);
    raise notice 'txn 1 for share %', c;
    delete from t1; -- wait multixid(trxn 1 and trxn 2) and update
end;
/

begin
    perform * from t1 for update;
    pg_sleep(1.5);
    raise notice 'txn 2 for update';
    update t1 set c1 = c1 + 1;
end;
/

declare
    c int;
begin
    pg_sleep(1);
    select c1 into c from t1 for share;
    pg_sleep(0.5);
    raise notice 'txn 3 for share %', c;
    pg_sleep(2);
end;
/

\parallel off

select * from t1; /* 0 rows */
drop table t1;


/* Test 3 */
create table t1 (c1 int) with (storage_type=ustore);
insert into t1 values (0);

\parallel on 3

declare
    c int;
begin
    pg_sleep(1);
    select c1 into c from t1 for share;
    pg_sleep(1);
    raise notice 'txn 1 for share %', c;
    delete from t1; -- wait multixid(trxn 1 and trxn 2) and update
end;
/

begin
    perform * from t1 for update;
    pg_sleep(1.5);
    raise notice 'txn 2 for update';
    update t1 set c1 = c1 + 1;
end;
/

declare
    c int;
begin
    pg_sleep(1);
    select c1 into c from t1 for share;
    pg_sleep(0.5);
    raise notice 'txn 3 for share %', c;
    pg_sleep(2);
    raise exception '';
end;
/

\parallel off

select * from t1; /* 0 rows */
drop table t1;

/* Test 4 */
create table t1 (c1 int) with (storage_type=ustore);
insert into t1 values (0);

\parallel on 4

declare
    c int;
begin
    pg_sleep(1);
    select c1 into c from t1 for share;
    pg_sleep(1);
    raise notice 'txn 1 for share %', c;
end;
/

begin
    perform * from t1 for update;
    pg_sleep(1.5);
    raise notice 'txn 2 for update';
    update t1 set c1 = c1 + 1;
end;
/

declare
    c int;
begin
    pg_sleep(1);
    select c1 into c from t1 for share;
    raise notice 'txn 3 for share %', c;
    pg_sleep(2);
end;
/

declare
    c int;
begin
    pg_sleep(5);
    select c1 into c from t1 for update;
    raise notice 'txn 4 for share %', c;
    update t1 set c1 = c1 * 3;
end;
/

\parallel off

select * from t1; /* 0 rows */
drop table t1;

/* Test 5 subtrans */
create table t1 (c1 int) with (storage_type=ustore);
insert into t1 values (0);

\parallel on 3

declare
    c int;
begin
    pg_sleep(1);
    select c1 into c from t1 for share;
    pg_sleep(1);
    raise notice 'txn 1 for share %', c;
    delete from t1; -- wait multixid(trxn 1 and trxn 2) and update
exception
    when others then
        raise notice 'error';
end;
/

begin
    perform * from t1 for update;
    pg_sleep(1.5);
    raise notice 'txn 2 for update';
    update t1 set c1 = c1 + 1;
exception
    when others then
        raise notice 'error';
end;
/

declare
    c int;
begin
    pg_sleep(1);
    select c1 into c from t1 for share;
    pg_sleep(0.5);
    raise notice 'txn 3 for share %', c;
    pg_sleep(2);
exception
    when others then
        raise notice 'error';
end;
/

\parallel off

select * from t1; /* 0 rows */
drop table t1;
