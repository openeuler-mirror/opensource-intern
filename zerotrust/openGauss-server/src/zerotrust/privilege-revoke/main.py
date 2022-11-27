import sys
import time
from gauss import OpenGauss, gen_sql
from config import user, host, logfile

dbname = sys.argv[1]
grantee = sys.argv[2]
level = int(sys.argv[3])

def log(sql, err=None):
    global dbname
    timestamp = time.strftime('%Y-%m-%d %H:%M:%S', time.localtime())
    log_str = f'{timestamp} {user} {dbname} {host} [ZerotrustPy] '
    if err is None:
        log_str += f'LOG: Execute "{sql}"\n'
    else:
        log_str += f'ERROR: Except ERROR "{str(err)}" while Exec "{sql}"\n'
    with open(logfile, 'a') as f:
        f.write(log_str)


if __name__ == '__main__':
    flag = '0'
    sqls = gen_sql(grantee, level)
    try:
        sql = 'Connect to openGauss...'
        og = OpenGauss(dbname)
        for sql in sqls:
            og.exec(sql)
            log(sql)
    except IOError as e:
        log(sql, e)
        flag = '/'
    finally:
        og.close()
        print(flag)
