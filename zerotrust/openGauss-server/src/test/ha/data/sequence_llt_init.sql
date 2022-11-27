-- prepare
DROP TABLE IF EXISTS Serial_Table_1;
CREATE TABLE Serial_Table_1
(
	C1 SERIAL,
	C2 INT
)
DISTRIBUTE BY HASH(C1)  
;
DROP SEQUENCE IF EXISTS SEQ_LLT_1;
CREATE SEQUENCE SEQ_LLT_1;

DROP SEQUENCE IF EXISTS SEQ_LLT_CACHE500;
CREATE SEQUENCE SEQ_LLT_CACHE500 CACHE 500;

