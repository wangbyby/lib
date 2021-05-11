linux入门：http://training.eeworld.com.cn/video/14269
    工作：1.Linux程序员c/c++，java，php，jsp（Java Server Pages）
                    -Linux软件工程师pc
                    -Linux嵌入式开发
                2.linux系统管理员
    学习方法：
        第一阶段：linux平台上的开发，vi，gcc,gdb,make,jdk,tomcat,mysql...和Linux基本操作
        第二阶段：加厚c功底《c专家编程》/java
        第三阶段：unix《unix环境高级编程》
        第四阶段：linux应用系统开发/Linux嵌入式开发
        思考——实践——再思考——再实践...

        先how then why
        先框架 再细节
        做了才会 “做中学”
        适当的囫囵吞枣
        琢磨别人怎么做，而不是我认为该怎么做

        1973 unix: ibm-aix, sun-solaris, hp-hpunix, bsd,minix-->pc 1991->1994 linux(1.0)  
    linux :
    缺点：相对麻烦
    优点：1.free 2.多线程/多用户 3.安全性 4.对内存和文件管理优越
    操作：关机： shutdown -h now
                shutdown -r now
                reboot

        vi编辑器：开发步骤：
        1.vi hello.java
        2.输入 i 进入插入模式
        3保存：输入esc 进入命令模式
            输入' ：' wq #保存退出  q! 退出不保存
        4. ls -l
        5.编译java：javac hello.java
        6运行 ：java Hello
        linux 开发c/c++

    ./a.out(c程序)
    <vi编辑器的使用>
    linux文件目录：
    级层树状目录结构
    最上层是根目录“/”,然后再目录下创建其他目录。
        /：根目录
            1.root目录
                root用户的文件
            2.home目录
                普通用户的文件
            3.bin
                普通命令的目录
            4.sbin
                要有一定权限才能使用的命令
            5.mnt
                默认挂载软驱和光驱
            6.etc
                配置相关文件
            7.var
                经常变化的文件
            8.boot
                引导文件
            
    显示当前目录：pwd
    相对路径：以当前目录为起点，去寻找某个文件（夹）
    绝对路径：从根目录去访问某个文件（夹）

    工作目录：默认的根目录

    用户管理：
        添加用户命令： useradd xiaoming
        设密码： passwd xiaoming
        删除     userdel 用户名 （删除用户）
                userdel -r 用户名（删除用户及主目录）
    运行级别：
        命令： init [0123456]
        0：关机
            1：单用户
            2：多用户没有网络
            3：多用户有网络
            4：系统未使用保留给用户
            5：图形界面
            6：系统重启
            3,5常用
            /etc/inittab id：5：initdefault
        解决修改错误配置的方法
                进入grub引导界面时，输入e
                选中第二行 输入e 
                再输入  1（单用户级别）
                    再去修改运行级别
        pwd 显示当前工作目录
        cd  该变目录
            cd（只有cd）   : 到用户的目录
        ls 
            ls -a(隐藏文件)
            ls -l（长列表文件）
            ls -ahl 文件所在组
        1.mkdir :建立目录
        2.rmdir ：删除空目录
        3.touch :建立空文件
        4.cp : 复制命令 
            cp -rf 源 目标 （从 源 到 目标）
        5.mv : 移动文件和改文件名
        6.rm : 删除文件 
            rm -rf 目录名
        7.ln ：建立符号链接
            ln -s 源 目标 (目标->源)
        8.more ：显示文件内容
        9.less ：分页
        10.grep ：文本中查找关键词
            grep -n
        11.| :管道命令 再linux和unix中（把上一个命令的结果交给| 后的命令处理）
        12.man ：帮助
        13.find：搜索文件及目录
            find 目录 -amin -10 （十分钟内）
            find 目录 -amin  +10 （十分钟前）
                a：内存  c：硬盘
                min 分钟 time 小时
        14.重定向： ls -l > a.txt(覆盖写)
                    >> (追加写)
    组：一个用户可在多个组中
        添加组：groupadd 组名
        查看组的信息： cat(vi) /etc/group
        创建用户，并同时指定该用户分配到组中： useradd -g 组名 用户名
        查看所有用户信息 ： cat（vi） /etc/passwd 
        -rw-r--r--
        -   rw-     r--     r--
            -:(文件类型)
                d：文件夹
                - ：普通文件 
            rw- ： 文件所有者对该文件的权限
                
            r-- 
                文件所在组对该文件的权限
            r--
                其他用户对该文件的权限
            r：可读 （4）
            w ：可写（2）
            x ：可执行（1）
        修改文件文件访问权限： 
                chmod 777 用户名 
                7 代表 r w x xiaoming
        改变组：
            usermod -g 组名 用户名
    linux java环境
        1).jdk
            1.把文件 挂载 到linux上
                卸载：umount 目录
            2.把安装文件拷贝到 /home
                cp 文件 /home
            3. cd /home
            4. 安装
                ./ 文件名.bin
            5.看一个文件 /etc/profile （环境配置文件）
        tar.gz的安装：
            tar -zxvf ？？？.tar.gz
        启动：(进入图形界面 ：startx)
            cd vscode
            ./vscode
            { myeclipse java ee weblogicwebsphere}
            & ： 后台运行
    安装realpaly
        1.把安装文件拷贝到 /home
        2.安装 ./ 文件名.bin
    硬盘分区：
        基本分区
        扩展分区: 分成逻辑分区才能使用
        一个硬盘中：扩展分区 + 基本分区 <= 4
    linux ：
        硬盘分区 ：挂载在boot目录上

        fdisk -l 分区情况 df -l 磁盘使用情况
        /dev/sda(loop) 
            sd:企业级 hd：普通用户
            a：盘号
            1：第一个分区 1-4 为主分区 5以后为逻辑分区
    一个独立且唯一的文件系统 根目录“/”

    shell: 《linux命令，编辑器和shell编程》
        操作：
            ctrl + w 清楚光标前的单词
            ctrl + u 清空行
            ctrl + c 终止当前命令
            ctrl + t 调换光标前的两个单词
            ctrl + r 搜索先前使用的命令
            ctrl + d 退出shell
        种类：
            Bourne   /bin/sh
            C        /bin/csh
            Kom      /bin/ksh
        1.env 显示当前环境变量
        2.chsh -s 输入新的shell(shell 的修改) EX： /bin/csh
        3.命令补全
            按两次tab键
        4.历史纪录
            history
                history 5 最近5个命令
                ！5  执行历史编号为5的命令
        5.
            /etc/bashrc 基本配置数据
            /etc/profile 系统环境变量
        6.配置.bashrc文件让用户bai自启动
            vi /home/bai/.bashrc
        7.临时加入一个路径
            export PATH=$PATH:/root
        8.显示变量内容
            echo 
        9.取别名
            alias llh='ls -l /home' (等号前后无空格)
        shell编程：
            变量：
                系统变量（环境变量） / 用户自定义的变量（本地变量，shell变量）
            变量创建：语法：varName=varValue （‘=’左右不能用空格）
            
    通配符：
        1. * 多个字母或数字
        2. ? 一个字母或数字

        ipconfig（Windows） 
        ifconfig（linux/unix）
        tracert 目标ip/域名 ：追踪路由命令
    ping    目标ip/域名： 测试两个目标是否畅通
            127.  .  .  回路
        子网：
            前面网络号一样 就在一个子网内
        代理：
            你懂的
    linux网络环境配置：
        1. 
            1.root登陆
            setup 
            text mode setup utiliy
            2. /etc/rc.d/init.d/network restart
        2. 
            /etc/sysconfig/network-scripts/ifcfg-eth0

            /etc/rc.d/init.d/network restart
    RPM管理： 公认标准
        apache-1.3.23-11.i386.rpm
        apache:软件名称
        1.3.23-11 :版本号，主版本次版本
        i386：硬件
        rpm -qa （查询安装了那些rpm软件）
        安装rpm
            rpm -i RPM全路径
            rpm -ivh RPM全路径
    samba:
        1.安装
            是否安装？ rpm -q samba
        2.把自己的安装文件挂载在linux下
        3.把 samba 拷贝到/home 
            cp /mnt/cdrom/samba* /home
        4.创建一个用户 youyou
            useradd youyou
            passwd youyou
        5.youyou设置samba密码
            cat /etc/passwd | mksmbpasswd.sh > /etc/samba/smbpasswd
            smbpasswd youyou
        6.使用
            service smb start
            service smb stop
            service smb restart
    任务调度：系统在特定时间执行特定命令
    crontab
        1.设置任务
            crontab -e
        2.每隔一定时间执行
            date > /home/mydata1
        希望每天凌晨2：00执行data >> /home/mydata2
            crontab -e 中加入
            0 2 * * * data >> /home/mydata2
        希望每分钟
            * * * * * data >> /home/mydata2
        *（每小时第几分钟） *（每日第几小时） *（每月的几号） *（每年的几月） *（每周第几天）
        data MMDDHHMMCCYY.SS 设时间
        cal year
        3.调动多个任务
            1.在crontab -e 直接写
            2.把所有任务写入一个可执行文件（shell）
        4.终止任务
            crontab -r
            crontab -l
            
    Linux进程：正在执行的程序（有独立的地址空间）
        前台：屏幕上可以操作
        后台：实际在操作，屏幕上看不到（常驻在内存中）
        ps -a（u/x）
        ps -aux
    终止进程：
        kill 进程号
        kill -9 进程号（加强版）
        killall：进程以及子进程都杀掉
    动态监控：top
        k：每隔多长时间刷新
        u :监视
    线程：   轻量级的进程
            没有独立的地址空间
            线程不能独立存在，线程由进程创建
            相对讲：线程耗费的cpu，内存小于进程
    监控网络状态
        1.netstat -anp 
        2.traceroute
        3.route
    mysql：
        1.安装：
        2.解压缩 
        3.
            groupadd mysql
            useradd -g mysql mysql
            进入mysql文件夹
                scripts/mysql——install——db --user=mysql
            chown -R root .     -R(递归) . : 当前目录
            chown -R mysql data
            chgrp -R mysql .
            bin/mysql_safe --user=mysql &  【& 以后台方式启动】
        4.如何进入
            1.  cd bin
                ./mysql -u root -p
            2.
            vi .bash_profile
        5.mysql使用：
            进入mysql
            creat database 名字
            create table users(userId varchar(8),userName varchar(8));
            结合Java
                mysql驱动 放到jdk下 主目录/jre/lib/ext/
            备份：
                mysqldump -u root -p密码 数据库名 > 目录
            恢复：(建立一个空数据库)
                mysql -u root -p密码 数据库名 < 目录
    ssh客户端(secure shell)
    1.默认linux安装 sshd 监听22端口 
    2.windows安装
    3.linux启动：
        使用ssh登陆到linux
    linux 启动：
            1.BIOS自检
            2.启动GRUB/LILO
            3.运行Linux内核并检测硬件
            4.运行系统第一个进程init
            5.init 读取/etc/inittab 的信息
            6./etc/rc.d/rc.sysinit 系统初始化脚本
            7./etc/rc.d/rcX.d/[KS]* - 根据运行级别X配置服务
                终止“K”开头的服务
                启动“S”开头的服务
            8./etc/rc.d/rc.local 执行本地特殊配置
            9.其他
    zip和uzip：
        zip aa.zip 文件名           //压缩单个
        zip aa.zip 文件1 文件2     //压缩多个
        zip -r aa.zip 文件夹路径    //压缩文件夹
        uzip 压缩包名

    gzip和ugzip处理.gz文件
        unzip file.zip
    linux安装：
        1.独立安装
        2.虚拟机安装
    tcp（传输控制）/ip（网络）:一组协议
        1.history   1972华盛顿会议，制定通用通讯协议
                    1974美国国防部共享
        2. 7层协议（理论）
            4层（现实）应用，传输，网络，链路
            书籍：协议，实现，TCP事务协议

    linux下java网络编程
        //server
        import java.io.*;
        import java.net.*;
        public class MyServeTest {
                public static void main(String []args){  
                    try{
                        ServerSocket ss = new ServerSocket(8888);
                        //监听，直到客户端链接 一旦链接成功，就得到Socket 
                        Socket s = ss.accept();
                        InputStreamReader isr =  new InputStreamReader(s.getInputStream());
                        BufferedReader br = new BufferedReader(isr);
                        String kk = br.readLine();
                        System.out.println(kk);   
                        
                        //从服务器端，回一个消息
                        OutputStreamWriter osw = new OutputStreamWriter(s.getOutputStream());
                        PrintWriter pw = new PrintWriter(osw,true);              
                        pw.println("I get your message"+kk);
                    }catch (Exception e){
                        e.printStackTrace();          
                    }   
                }
        }
        
        //client  
        import java.io.*;
        import java.net.*;
        public class MyClient {
            public static void main(String []args){  
                    try{
                        Socket s = new Socket("192.168,222,66",8888);
                        //通过文件流发送消息
                        OutputStreamWriter osw = new OutputStreamWriter(s.getOutputStream());
                        BufferedWriter bw = new BufferedWriter(osw);
                        PrintWriter pw = new PrintWriter(bw,true);
                        //发送
                        pw.println("hello world");
                        
                        //读取从服务器回送的信息
                        InputStreamReader isr =  new InputStreamReader(s.getInputStream());
                        BufferedReader br = new BufferedReader(isr);
                        String res = br.readLine();
                        System.out.println("server response is "+res);   
                    }catch (Exception e){
                        e.printStackTrace();          
                    }   
                }
        }
