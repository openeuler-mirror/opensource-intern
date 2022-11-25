SELECT SESSION_USER, CURRENT_USER;
reset session AUTHORIZATION;

create user user1 PASSWORD 'Gauss123';
create user user2 PASSWORD 'Gauss123';
SET SESSION AUTHORIZATION user1 password 'Gauss123';
drop procedure p1;
create procedure p1
is
begin
null;
end;
/
drop package if exists pck1;
create or replace package user1.pck1 as
procedure p1();
end pck1;
/

--包内嵌套定义
create or replace package body pck1 as
procedure p1 is
begin
null;
end;
end pck1;
/

SELECT SESSION_USER, CURRENT_USER;
reset session AUTHORIZATION;
SELECT SESSION_USER, CURRENT_USER;
---修改 package owner
alter package user1.pck1 owner to user2;

---校验
------usename 为 user2
select usename from pg_user where usesysid = (select pkgowner from gs_package where pkgname = 'pck1');

grant usage on schema user1 to user2;
grant execute on package user1.pck1 to user2;
------调用成功，结果正确
SET SESSION AUTHORIZATION user2 password 'Gauss123';
drop procedure p1;
call user1.pck1.p1();

------原owner create or replace 预期失败
SET SESSION AUTHORIZATION user1 password 'Gauss123';
create or replace package pck1 as
  type t1 is record(c1 int,c2 int);
  type t2 is table of t1;
  type t3 is varray(10) of t1;
  type t4 is ref cursor;
end pck1;
/

create or replace package body pck1 as
  type t5 is record(c1 t1,c2 int);
  type t6 is table of t5;
  type t7 is varray(10) of t1;
  type t8 is ref cursor;
end pck1;
/

reset session AUTHORIZATION;
SELECT SESSION_USER, CURRENT_USER;
select usename from pg_user where usesysid = (select pkgowner from gs_package where pkgname = 'pck1');

---清理
SET SESSION AUTHORIZATION user1 password 'Gauss123';
drop package if exists pck1;
reset session AUTHORIZATION;
drop user if exists user1 cascade;
drop user if exists user2 cascade;
