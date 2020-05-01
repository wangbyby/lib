## 二.进阶
    变量定义
        会话变量：
                属于某一客户端的变量（不同客户端之间相互独立）
                show session variables;
                set 变量名='';
                set @@session.变量名='...';
        全局变量：
                针对数据库
                show global variables;
                set global 变量名='....';
                set @@global.变量名='''';
                select @@global.变量名;
    存储过程
        多个sql语局放在一起执行
        优点
            灵活/快/减少网络流量/安全机制
        存储过程的创建：    
            1.选中数据库
            2.改变分隔符
                不让';'作为执行结束的标记
                delimiter $$; 
            3.创建
                create procedure 过程名()
                    begin
                        .....; //以;结尾
                    end
                    $$; //drop procedure 过程名;
            4.恢复分割符
                恢复delimiter ;
            5.调用        
                call 过程名
        局部变量的定义
            declare 变量名 数据类型 default 默认值;
            参数：
                in：输入参数 
                    在存储过程中修改的值，不能影响到存储过程外原有变量
                out：
                    在存储过程中修改的值，能返回
                    在外赋值一直为null(不认可之前的赋值)
                inout：
                    既能赋初值
                    又能返回
                EX.//create procedure test_0(in 变量名 类型)
                        begin 
                            expr
                        end 
                        分隔符
                    //set @p=10;
                    //call test_0(@p);
        流程控制：
            while  语法：
                    while expr do 
                    语句
                    end while;
            repeat  语法:
                    repeat
                        内容
                    until 条件
                    end repeat;
            loop  语法：
                    名字: loop 
                        内容
                    if expr then
                        leave 名字;
                    end if;
                    end loop;
        定义条件和处理：
            //类似异常
            语法： declare continue handler for sqlstate '错误代码值' 
                    set 变量=变量值;
        存储过程管理
            show procedure status where db='数据库名';
            当前存储过程的列表：
                select specific_name from mysql.proc;
            查看存储过程的内容
                select specific_name,body from mysql.proc;
                或 show create procedure 存储过程名;
            删除
                drop procedure if exists 过程名;
            修改：
                alter procedure 存储过程名 参数;
    触发器
        一种特殊的存储过程
        监视地点：一般是表名
        监视事件：update/delete/insert
        触发时间：after/defore
        触发事件：update/delete/insert
        //不能直接调用,满足条件自动执行
        创建语法：
            create trigger trigger_name trigger_time trigger_event
            on table_name for each row 内容;
        update: 修改前的数据用old来表示 old.列名引用被修改之前行中的值
                修改后，的数据用new来表示 new.列名引用被修改之后行中的值
        管理：
            show triggers;
            desc information_schema.TRIGGERS;
            drop trigger 表名.触发器名;
    锁 
        处理稀缺资源 //对有限资源进行保护，解决隔离和并发的矛盾
        表级锁/行级锁/页面锁
        myisam
            共享读锁：//都能读
                lock table 表名 read;//lock tables
                unlock table 表名 read;//unlock tables
            独占写锁：//只有一个能读
                lock table 表名 write;//lock tables
                unlock table 表名 write;//unlock tables
            并发插入
                concurrent_insert = 0(不允许),1(默认),2(在表尾插入)
    慢查询
        超过指定时间的SQL语句查询称为'慢查询'
        show variables like '%long%';
            long_query_time 
        status：
            show status like '%uptime%';
            show status like 'com_Select';
            show status like 'connections';
        启动慢查询
            安全模式
    索引
        在关系型数据库中，建立在表中的一种数据结构，使表中sql语句执行更快
        查看索引
            show keys from 表名;
            show index from 表名;
        分类
            主键索引
            唯一索引
                所在的列不能为空字符串，可以为null
                create unique index 索引名称 on 表名;
            普通索引
                create index 索引名称 on 表名;
            全文索引 //只有myisam支持
            explain分析sql的执行计划
                explain 语句;
        原理：二叉树
        分析 profile
            select @@have_profiling;
            set profiling = 1;
            show profile for query queryID;
    表
        分析表
            analyze table 表名;
        检查表
            check table 表名;
        优化表
            optimize  table 表名;
            //只对myisam，innoDB，BDB起作用
        //在执行时都对表进行锁定
        //show table status 获取表的信息
    分区  //数据量过大时，分划到几张表中
        range分区  
            创建
                语法：
                create table 表名(
                    ...
                )
                partition by range(列名)(
                    partition p0 values less than(.),
                    partition p1 values less than(..),
                    partition p2 values less than(....),
                    ...
                    partition pn values less than MAXVALUE
                );//主键必包含在分区中
        list分区 
            枚举型
            partition by list(列名)(
                    partition p0 values  in (....),
                    partition p1 values  in (...),
                    partition p2 values  in (..),
                    ...
                );
        hash分区 
            partition by hash(列名)
            partiton 数量;
        keys分区 
        管理：
            删除/增加
    优化
        内存
            myisam
                使用key_buffer缓存索引块
                每个session独有的
                    read_buffer_size
                    read_rnd_buffer_size 影响 order by语句
            innodb
                用内存区做io缓存池，缓存索引块和数据块
            并发相关
                max_connections 
                thread_cache_size 
        应用
            1.访问时采用连接池
            2.采用缓存减少对mysql的访问
            3.负载均衡
                主服务器(写)与从服务器(读) 分离
    权限管理
        use mysql;
        select host,user from user;
        show grants for 'root@localhost';
        账号名@主机名
        //主机名 ip地址/localhost
        赋权限
            grant select on *.* to test@localhost indentified by 'root' with grant option;
    监控：
        自己写程序/采用商业解决方案/开源软件
    定时维护
        mysql定时器 5.1之后版本
            show variables like '%sche%';
            set global event_scheduler = 1;
        创建
            create event 名字 on schedule every 大小 second（时间单位） on completion preserve disable 
            do call 存储过程名;
        开启（关闭） ： alter event 定时器名 on completion preserve enable（disable）;
    备份还原
        mysqldump:
            原理    
                1.查询出表的结构
                2.创建表
                3.insert语句，插入数据
            语法：
                mysqldump -u root -p 数据库名 table1，table2 > 目录/文件名.SQL   //在命令行下执行
                mysqldump -u root -p 数据库名  > 目录/文件名.SQL        //备份所有表
                mysqldump -u root -p --all-databases > 目录/文件名.SQL  //备份所有数据库
        还原：
            mysql -u root -p 数据库名 < 目录/名字.SQL //版本号相同
