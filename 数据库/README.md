
# win下安装mysql

```bash
# 1. 切换目录
D:\#codings\mysql-5.7.30-winx64\mysql-5.7.30-winx64/bin

# 2.输入安装命令
mysqld install
# 3. 初始化数据库
mysqld --initialize --console

# 4. 记录输入root用户的初始默认密码

root@localhost: yoHl!%+EZ3y

# 5.启动
net start mysql
```

## 登录mysql

``mysql -h 主机名 -u 用户名 -p``

本机:``mysql -u root -p``

## 问题

- ``net start mysql``执行时: 发生``系统错误 2``。
  - 解决方法
    - ``mysqld --remove``
    - ``mysqld --install``
    - ``net start mysql``
- ``net start mysql``发生``系统错误 5``
  - 管理员权限运行``net start mysql``

- win忘掉密码
  - ``net stop mysql`` 关闭mysql服务
  - ``mysqld -nt --skip-grant-tables`` 跳过安全检查
  - 保持cmd命令窗1打开状态，新开cmd命令窗2
  - ``mysql`` 连接
  - 更新密码 
    - use mysql;
    - select user,authentication_string,host from user;
    - update user set authentication_string = password``('root')`` where user = 'root' and host = 'localhost';
    - select user,authentication_string,host from user;
    - flush privileges;
-  You must reset your password using ALTER USER statement before executing this statement.
   -  ``alter user user() identified by 'root';``