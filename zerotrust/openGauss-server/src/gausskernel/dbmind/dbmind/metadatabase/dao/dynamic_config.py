# Copyright (c) 2020 Huawei Technologies Co.,Ltd.
#
# openGauss is licensed under Mulan PSL v2.
# You can use this software according to the terms and conditions of the Mulan PSL v2.
# You may obtain a copy of Mulan PSL v2 at:
#
#          http://license.coscl.org.cn/MulanPSL2
#
# THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
# EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
# MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
# See the Mulan PSL v2 for more details.
"""`dynamic_config_set()` and `dynamic_config_get()` startup after get_session().
And the function get_session() can only load SQLite database in the current working directory.
Hence, if we want to use `dynamic_config_set()` or `dynamic_config_get()`, we should change the
working directory to the confpath (the path of configuration files).

    Examples
    ------------
    >>> import os
    >>> os.chdir(confpath)
    >>> dynamic_config_get('foo', 'bar')

If you want to add more dynamic configurations, you should follow the underlying list:

1. Create a python file named config_xxx.py in the module of ```dbmind.metadatabase.schema```;
2. Define an ORM class for your dynamic configurations. You can refer to class ```DynamicConfig```.
3. Link your ORM class here. Add table name into the ```table_mapper```;
4. Then, all main processes are finished. You can call ```dynamic_config_set()``` and ```dynamic_config_get()```
functions to modify and read them.
"""
import logging

from sqlalchemy import update, insert
from sqlalchemy.exc import DBAPIError

# Register dynamic config table here.
from ..dynamic_config import get_session
from ..schema.config_dbmind_system import DBMindConfig
from ..schema.config_detection_params import DetectionParams
from ..schema.config_iv_table import IV_Table
from ..schema.config_slow_sql_threshold import SlowSQLThreshold

table_mapper = {
    SlowSQLThreshold.__tablename__: SlowSQLThreshold,
    DBMindConfig.__tablename__: DBMindConfig,
    IV_Table.__tablename__: IV_Table,
    DetectionParams.__tablename__: DetectionParams
}


def dynamic_config_set(table_name, name, value):
    table = table_mapper.get(table_name)
    if table is None:
        raise ValueError()

    with get_session() as session:
        # If the table has the given name, then update its value.
        # Otherwise, insert a new row into the table.
        if session.query(table).filter(table.name == name).count() > 0:
            session.execute(
                update(table).where(
                    table.name == name
                ).values(
                    value=value
                ).execution_options(
                    synchronize_session="fetch"
                )
            )
        else:
            session.execute(
                insert(table).values(name=name, value=value)
            )


def dynamic_config_get(table_name, name):
    table = table_mapper.get(table_name)
    if not table:
        raise ValueError('Not found the table %s.' % table_name)
    with get_session() as session:
        try:
            result = tuple(session.query(table).filter(table.name == name))
        except DBAPIError as e:
            logging.exception(e)
            # Return a none value instead of raising an
            # error directly because we have a fault-tolerant mechanism
            # later, giving the fault-tolerant mechanism a chance to try.
            return None
        if len(result) == 0:
            return None
        return result[0].value


def dynamic_configs_list():
    __no_need_for_showing__ = (
        DBMindConfig, IV_Table
    )
    rv = {}
    with get_session() as session:
        for table_name, table in table_mapper.items():
            if table in __no_need_for_showing__:
                continue

            rv[table_name] = tuple(map(
                lambda r: (r.name, r.value), session.query(table.name, table.value)
            ))
    return rv
