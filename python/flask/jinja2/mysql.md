
# 新建数据库

``名字别写错...``

# 报错

- No module named 'MySQLdb'
  1. ``pip install pymysql``这个比较靠谱 ~~相对于 ``mysqlclient``~~
  2. ``mysql+pymysql://username:password@server/db``
  3. 或者 ~~win下``pip install mysqlclient`` 不怎么好用~~

- sqlalchemy.exc.OperationalError: (MySQLdb._exceptions.OperationalError) (1045, "Access denied for user 'root'@'localhost' (using password: YES)")
    0. 我的目录结构 powershell ``tree /f``

    ```bash
    C:.
    │  bin.sh
    │  main.py
    │
    ├─config
    │  │  config.py
    │  │  db.sql
    │  │  init.sql
    │  │  __init__.py
    │  │
    │  └─__pycache__
    │          config.cpython-36.pyc
    │          __init__.cpython-36.pyc
    │
    ├─controllers
    ├─models
    │      users.py
    │
    └─templates
            404.html
            500.html
            base.html
            index.html
            user.html
    ```

    1. 在config文件夹下编写``config.py``文件, 内容如下: 

    ```python
    # ./config/config.py
    DIALECT = 'mysql'
    DRIVER = 'pymysql'
    USERNAME = 'root'
    PASSWORD = 'root'
    HOST = '127.0.0.1'
    PORT = '3306'
    DATABASE = 'bookstore'

    class Config:
        SQLALCHEMY_DATABASE_URI = '{}+{}://{}:{}@{}:{}/{}?charset=utf8mb4'.format(
            DIALECT, DRIVER, USERNAME, PASSWORD, HOST, PORT, DATABASE
        )
        SQLALCHEMY_COMMIT_ON_TEARDOWN = True
        SQLALCHEMY_TRACK_MODIFICATIONS = False

        SQLALCHEMY_POOL_SIZE = 10
        SQLALCHEMY_MAX_OVERFLOW = 5


    ```

    2. 在``main.py``中

    ```python

    from config.config import Config

    app = Flask(__name__)
    app.config.from_object(Config)

    app.config['SECRET_KEY'] = 'hard to guess string'
    bootstrap = Bootstrap(app)
    moment = Moment(app)
    db = SQLAlchemy(app)

    ```
