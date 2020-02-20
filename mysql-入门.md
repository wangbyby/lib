
# mysql入门
## 一.入门
    1.记录
        记录的都是信息
    2.信息时代：
        更高效记录信息的方式
    3.数据库管理系统/服务器
        [数据库！=mysql，满足国际sql标准(未必100%遵循)]
        多个库
            库里多个表
        学习mysql的原因
            1.mysql与linux紧密
            2.开源，免费
            3.postgresql
            4.叛本，不追求最新版本 {开源软件 有beta版，stable版，尽量用stable版}
        mysql命令行与mysql服务器什么关系？
            客户端 client --》 服务端 server的关系
        其他mysql客户端：
            navcat， mysqlfront
    4.十个基本语局
        0.连服务器 mysql -u root -p
            (首先面对多个库) 
            1.查看所有库  
                show databases; (记得打';'号)
            2.创建库 
                create database 数据库名 chatset(字符集)；
            3.删除库 
                drop database 库名；        
            4.选库：
                use 库名; (选完库后，面对表)
            5.查看表（先选择库）
                show tables；
            6.简单的建表  
                create table stu(num int, name varchar(10))engine myisam charset utf8;
                //engine  引擎与性能相关
            7.删除表  
                drop table 表名； 
            8.表改名 表/列可以改名， database不能改名
                rename table oldname to newname;
            9.插入数据
                insert into stu values (1,'zhangsan'),(2,'lisi'),(3,'wangwu');
                    select * from 表名;
            10.清空表数据   
                truncate 表名;
                    truncate    相当于删表再重建一个空的表
                    delete      删除所有数据行
                    全清空truncate更快点
    5.基础增删改查(针对单个table)
        tee 目录名，把sql及结果都输出到一个sql文件里
        1.乱码？客户端与数据库字符集类型不同 
            set names 字符集名; 
        desc 表名；//查看表的信息
        3.增删
            增：
                insert into 表名 (表头列表) values (值列表), (list2); //注意不要忘了写表名
                添列：alter table 表名 add 列名 数据类型;
                EX. insert into class values (1,'张三','男','百度', 8888.88, 200);
            删：删除整行，不存在一行的某几列
                delete from 表名 where expr;
        4.改查
            改：
                update 表名 set 表头= ? where expression;
                    EX: update 表名 set name = '' where id = ?(表达式);
                where后表达式:
                为真 则该行发挥作用 
                (改哪张表 哪几列 哪一行生效 改成什么值)   
            查：
                select 列名list（可倒序） from 表名 where expr;
                暴力查找： select * from 表名;//不要随便用
                导入数据库：source 目录
    6.建表 (声明字段的过程)：
        1.列类型：
            1.数值型
                整型  tinyint/smallint/mediuint/int/bigint
                    tinyint 
                        {占据空间： 1字节(8个位)
                        范围 -128~127，0~255(unsigned)}
                    参数：(M),unsigned,zerofill
                            默认有符号
                        unsigned 无符号
                        M：宽度
                        zerofill 不够位数，0填充 （必是unsigned类型）
                            M必须与zerofill配合才有意义
                浮点型
                    float(M，D) 10^38 ~ 10^-38 （M<=24  4个字节 M>24 8个字节） 
                    decimal(M,D) 整数部分和小数部分分开存储，比float精确
                        M:  小数总位数（不包括小数点）精度 
                        D:  小数点后位数             标度 
            2.字符型
                char ，varchar， text， blob
                char(n): 定长字符串
                    不够n，在尾部用空格补齐
                    (都是定长，查找时，可用行数计算出偏移数)
                varchar(n):             
                    前面有长度标识符，不用空格补齐，但列前有标注占1~2字节
                注意：char(M)， varchar(M) 限制的是字符数，不是字节数    
                text ：文本，存储文章，新闻内容
                    不必给默认值
                blob：二进制类型，存储图像，音频等信息
                    意义： 防止因字符集问题导致信息丢失
            3.日期时间
                date 日期 1000-01-01 到 9999-12-31
                time 时间 00:00:00
                datetime  YYYY-MM-dd HH:mm:ss
                year 1901-2155  (特殊：0000)
                timestamp  当前时间 default CURRENT_TIMESTAMP
            4.null类型 //空 
                Q：为什么建表时，default ''/0 not null？
                A: 不想出现null
                Q：为什么不想要null？
                A： 1.比较要用特殊运算符 is null,is not null 碰到运算符，都返回null
                    2.效率不高，影响索引效率
                因此建表时 default ''/0 not null 
        2.创建表
            声明列过程
            create table 表名(
                列1名 列类型 列1参数，
                列2名 列类型 列2参数，
                ......
            )engine myisam/innodb charset utf8/gbk;
            [把频繁用到的，考虑效率存到一张表
                不常用和交占据空间的，存到另一张表中]
            EX1.   create table class(
                    id int auto_increment not null  primary key ,
                    name varchar(10)  default '' not null,
                    gender char(1)  default '' not null,
                    company varchar(20)  default '' not null,
                    salary decinal(6,2)  default 0.00,
                    fanbu snallint  default 0
                )engine myisam charset utf8;    
            EX2. create table website(
                id int unsigned auto_increment not null primary key,
                name varchar(15) default '' not null,
                gender char(1) not null,
                weight tinyint unsigned default 0 not null,
                birth date not null,
                salary decimal(8,2) default 0.0 not null,
                lastlogin datestamp default CURRENT_TIMESTAMP not null,
                intro varchar(100) deafault '' not null
            )engine myisam charset utf8;
                name和intro之外，其他都是定长的
                为提高查询速度，让所有列都定长
            优化: 
            create table website(
                id int unsigned auto_increment not null primary key,
                name char(15) default '' not null,
                intro varchar(100) deafault '' not null
            )engine myisam charset utf8;
            create table website(
                id int unsigned auto_increment not null primary key,
                gender char(1) not null,
                weight tinyint unsigned default 0 not null,
                birth date default '1000-01-01'not null,
                salary decimal(8,2) default 0.0 not null,
                lastlogin int
            )engine myisam charset utf8;
                name 浪费部分空间，但提高速度  值
                intro 简介 一旦注册改的频率不高
                把频繁用到的，考虑效率存到一张表
                不常用和交占据空间的，存到另一张表中
    7.列的增删改
        1.增 alter table 表名 add 列名称 列类型 列参数;
            alter table 表名 add 列名称 列类型 列参数 after 某列;
            //新建一个列，并在最前面 alter table 表名 add 列名称 列类型 列参数 first;
        2.删 alter table 表名 drop 列名;
        3.修改列类型  
            alter table 表名 modify 列名 新列类型 新列参数;
            （不能该改列名）
        4.修改列名及列类型
            alter table 表名 change 旧列名 新列名 新列类型 新列参数;
            //不匹配数据会丢失或修改不了
    8.查
        where 条件查询(针对 表 发挥作用)
            运算符：
                in 在某集合内
                between 在某区间内
                != 或 <> 不等于
                    () : 集合
                    between ...and...
            模糊查询
                like 
                    %:通配符
                    _:匹配任意单一字符
            查询模型：
                列看成变量，where看成 if 表达式
                列之间就能运算，术语 广义投影
                起 列别名  (...) as ....
            EX.where发挥作用时，表上并没有discount列，发挥完作用，形成的结果里才能discount
            substring(),concact()
        group by 分组与统计函数
            统计函数: max，min,sum,avg,
                    count(*)，count(1)//绝对行数 
                    count(列名) //该列不为null的行数
                    即count(0/false)也会计数
                    //count(*),count(1) 谁好呢？
                        对于myisam引擎，没有区别
                        对于innodb的表，count(*)效率低
            group by 分组查询
                group by 列名;
                //严格的讲，select a,b,c  group 后 从a b c中选择
        having 筛选
            筛选得到结果集
        order by 排序
            对结果集排序
                desc：降序排列
                默认：升序
            多个字段用 ',' 隔开
                EX.列1 desc/asc，列2 desc/asc ....
            //内层order by 能影响结果集才有意义
        limit 限制结果条数
            [offset],N
            offset:默认为0 //跳过行数
            N：取出行数
        查询顺序：where > group by > having > order by > limit
        //    show create table 表名;
        //    批量导入：insert 库名.表1名 select 表头列表 from 库名2.表名2;
        子查询:
            where
                内层查询结果作为外层查询的比较条件
                如果 where 列 = (内层 sql)，则内层sql返回的必是单行单列，单个值
                如果 where 列 in (内层sql)，则内层sql只返回单列，可以多行
                select id,name from good where id in (select max(ca_id) from good order by id);
            from
                即内层sql的查询结果，当成一张临时表
            exist
                外层sql的结果，拿到内层sql测试，如果为真，取出该列
                select id,name from good where exists (select * from cat where cat.id =good.id);
    9.左连接，右链接，内链接
        1.表与集合的关系
            一张表就是一个集合
                每一行就是一个元素
        Q：两行数据一样
        A：mysql中有rowid区分
        数据库中的笛卡尔乘积： 
            select * from test0，test1;
            //列名重复 用表名.列名区分
            //没有利用索引
            //效率低
        2.左连接
            语法: 假设A表不动，B表在A表右边滑动
            A left join B on 条件
            //条件为真，B表对应行取出
            //形成一个结果集，可以看成一张表  A，B的列都可以查
            EX. select goods_id,cat_id from good left join cat on good.cat_id = cat.cat_id;
            //注意服务器内存占用
        3.右链接
            与左连接相反
        4.内链接
            左连接与右链接的交集 ∩
            语法：A inner join B on 条件
        5.外连接
            左连接与右链接的并集 ∪
            //mysql不支持
        Q:左连接，右链接，内链接的区别
        Answer: A left join B 等价于 B right join A
        //左右链接可以互换，尽量用左链接，出于移植兼容性的考虑
    10.union
        合并多条sql语句的结果
        语法： sql1 union sql2;
        //unoin合并的是结果集，不区分来自哪张表
        //要求：多张结果集的列数相同
        //列名不一致，列名以第一条sql的列名为准
        Q.union后结果重复
        A. 比较常见，默认去重
        union all 不去重
    11.mysql函数
        1.数学函数
            abs(x)//绝对值
            bin(x)//二进制
            floor(x)//对小数取整
            ceiling(x)//向上取整
            rand()//随机0-1.0的的随机值 
        2.聚合函数
            group_concat(col,x)
            //x为分割符 默认为',' 把col拼接到一起
        3.字符串函数
            ascii(char)//计算字符的ascii码
            bit_length(str)//字符串的比特长度
            char_length(str)//字符数
            lenth(str)//字符串的字节长度
            reverse(x)//反转字符串
            locate(substr, str)，position(substr in str)//substr的第一次出现位置（从1开始数）
            right(str,N) //从右数 截N个字符
        4.日期时间函数
            now()//当前时间datetime格式
            curdate() //当前日期
            curtime()//当前时间
            dayofweek(date)//date在一星期的第几天
            dayofmonth(date)，dayofyear(date)
            week(date)//date在某年的第几个星期
            month(date),
        5.控制流函数
            case 值 when 可能1 then 返回值1  
                    when 可能2 then 返回值2 
                    else 默认值 end
            if(exp1,exp2,exp3) //exp1为真 返回exp2，反之返回exp3 三元运算符
            if exp1 else exp2
            语法：  if condition1 then 
                        exp1
                    elseif ...
                    else ...
                    end if 
            ifnull(exp1,exp2) // exp1为null时，返回exp2，反之返回exp1
            //在存储过程中，加分隔符';'
        6.加密函数
            md5://不可逆
                md5(str) 
        良好的加密：
            1.不可逆
            2.碰撞性低
        7.系统调试函数
            user()//返回用户及所在主机
            database()//正在操作库名
            version()//mysql版本
        //注意事项
            1.以合理的表结构减少函数的使用
                EX. email 以@前后区分
            2.使用时：
                比如时间的格式化
                优先在业务逻辑层处理    
            3.查询时使用函数的弊端
                date_format(A)
                A列索引无法使用
            即对某列使用函数，则该列索引无法使用，影响速度
        8.自定义函数
            1.查看能不能创建函数
                show variables like '%fun%';
                开启：
                    set global log_bin_trust_function_creators = 1;
            2.创建语法
                create function 函数名(变量1 类型1, 变量2 类型2...) returns 数据类型
                begin
                    ....
                return 数据;
                end;
            3.删除
                drop function if exists 函数名;
    12.视图 //映射
        视图view：
            可以看成一张虚拟的表，是表通过某种运算的到的一个投影
        语法：
            create view 视图名 as select 语句;
        用处：
            1.简化查询 
            2.更精细的权限控制 
            3.数据多，分表是可以用到
        表与视图数据相互影响的关系
            Q.视图可更新？
            A.有些可更新 //可更新要求：视图数据和表的数据一一对应
            order by limit的结果不是一一对应的
            一一对应：def：表中的行只能计算出视图中确定的一行
        algorithm 
            三种模式：merge(合并) temptable(临时表) undefined(系统决定)
            决定建临时表还是合并语句
            //一些简单视图  并没有建临时表，而只是把条件存起来，下次来查询，把条件合并去查表
            语法： create algorithm = merge/temptable(不可更新)/undefined view 视图名 as 语句;   
        管理：
            存放位置： information_schema 数据库下的views表里 //G为以列表现
        with check option 
            更新视图的数据， 必须先满足视图条件，之后才能更新基表
    13.utf8/gb2312
        计算机： 0/1  人的世界：文字/图片/声音
        字符集：2进制编码 到 字符的映射
        acsii字符集 0xxx xxxx
        GB2312 
            如何兼任ASCII又能双字节表示中文
            干脆gb2312不占用0-127
            但中文组合数少 1w+
            事实上只有6763个汉字
        GBK
            第二位 小于128的也能使用
        //兼容性问题
        asci ：本地字符集
        unicode:世界通用码表
            用4个字节编号
            常用的集中在前655355个字符中
            只负责分配编号，而且都用4个字节分配编号
        在不改变编号的基础上 简化字节
        unicode transformation 
            一种压缩方式
            utf8：网络传输数据方式
                变长码 1-6个字节
                unicode-- utf8
                原文件--  压缩文件
                Q.如何确定字符的边界
                A.按头部1的数目
            Q.如何截取utf8无乱码
            A.从头开始取一个字节，计算连续1的个数
        容量上：gb2312 < gbk < utf8
        乱码如何形成
            1.转换出错（转码时信息丢失 不可修复）
            2.解码时与实际编码不一致(可修复)
    14.mysql编码
        客户端-->连接器-->数据库server-->连接器-->输出
            客户端发送的编码 set character_set_client = 编码方式;
            连接器使用的编码 set character_set_connection = 编码方式;
            返回数据的编码     set character_set_results = 编码方式;
            三者相同时 简写 set names 编码方式;
        编码大小:server>=connection>=client
        不乱码：
            1.正确指定客户端编码
            2.合理选择连接器编码
            3.正确指定返回内容编码
        utf8的bom问题
            windows记事本创建时，前面多了三个字节 来辨识编码信息
            解决：保存为 utf8 而不是 utf8+
        utf8，utf-8，UTF-8//一样的，别名而已
        面试题：中文截取无乱码
            如何判断一个utf8字符的字节数
            字符转换为二进制，然后判断
    16.存储引擎
        myisam/innodb/bdb/memory/archive
        mysql存储数据的不同方式
        事务的原子性
            start tansaction;
            ...
            commit;
            //有一个故障 就会回滚 rollback;
        一致性：
            操作前后值的变化，逻辑上成立
        隔离性：
            事务结束前，每一步操作带来的影响，别人看不见
        持久性：
            一旦完成，无法撤销
        myisam引擎：隔离性没有完成
    17.回顾
        站在数据库使用者
            学习mysql，增删改查，左右链接，子查询
        数据库设计者：项目经理，架构师，
            知识：引擎，索引，表的设计，读写分离
        数据库管理者DBA：
            权限管理，数据备份，运行监控，性能检测
