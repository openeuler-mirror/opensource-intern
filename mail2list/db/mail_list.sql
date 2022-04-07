create database mail2list;

\c mail2list;

BEGIN;


CREATE SEQUENCE sq_mail_id START 1;
CREATE TABLE mail_list
(
    id INT PRIMARY KEY,
    name varchar(20)  NOT NULL,
    email varchar(20),
    archive varchar(20),
    description varchar(100)
);

CREATE  TABLE "archive_mail_list" (
	id integer NOT NULL DEFAULT nextval('sq_mail_id'),
	name character varying,
	from_email character varying,
	create_time character varying,
	subject character varying,
	body character varying,
	filename character varying,
	message_id character varying,
	in_reply_to character varying,
	reference character varying
);


