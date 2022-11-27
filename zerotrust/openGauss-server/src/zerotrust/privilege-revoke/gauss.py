import psycopg2
from config import user, password, host, port


class OpenGauss:
    """
    This is a class for connectting to opengauss and executing SQL
    """

    def __init__(self, dbname):
        """
        When we initialize the class, we connect to opengauss
        """
        self.conn = psycopg2.connect(
            dbname=dbname,
            user=user,
            password=password,
            host=host,
            port=port,
        )
        self.cursor = self.conn.cursor()

    def exec(self, sql):
        """
        The main function to execute SQL
        """
        self.cursor.execute(sql)
        self.conn.commit()

    def query(self, sql):
        """
        Function to execute SELECT SQL for testing connection
        """
        self.cursor.execute(sql)
        rows = self.cursor.fetchall()
        return rows

    def close(self):
        """
        Close connection
        """
        self.conn.close()


def gen_sql(grantee, level):
    if level == 0:
        return []
    targets = []
    if level >= 1:
        t_sql = "REVOKE " \
                "DELETE,TRUNCATE,DROP " \
                "ON ALL TABLES IN SCHEMA public " \
                "FROM {};".format(grantee)
        targets.append(t_sql)
    if level >= 2:
        t_sql = "REVOKE " \
                "ALTER,UPDATE,INSERT " \
                "ON ALL TABLES IN SCHEMA public " \
                "FROM {};".format(grantee)
        targets.append(t_sql)
    if level >= 3:
        t_sql = "REVOKE " \
                "ALL PRIVILEGES " \
                "ON SCHEMA public " \
                "FROM {};".format(grantee)
        targets.append(t_sql)
    return targets
