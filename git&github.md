# 解决github每次push输入账号密码
1. git remote -v 

2. git remote rm <branch> #删除原先https连接
3. git remote add <branch> ssh地址 #连接方式更改为SSh
4. git push <branch>
5. 若有错误则 git push --set-upstream origin master 即把本地分支关联到远程主分支


# git使用笔记
git github
    1.版本控制 
        集中式: SVN,VSS,CVS
        分布式:    git
    2.git简介
    3.git命令行操作
    4.git图形化
    5.gitlab

1.版本控制 
    (1)思想
        个人
        团队
    (2)工具
    (3)功能:
        协同修改,数据备份,版本管理,权限控制,历史纪录,分支管理
    (4)git历史
        2005:git
        2008:github上线
    (5)git优势
        本地 (hash)
        完整性
        尽可能添加数据,而不是删除或修改数据
        分支操作快捷流畅
        与Linux命令全面兼容
2.git结构
    本地库--历史版本
    暂存区--临时储存
    工作区--写代码

    工作区 git add -->暂存区 git commit --> 本地库

    代码托管中心 维护远程库
        局域网: gitlab
        外网:
            github
            码云
3.本地库
    (1)初始化
        git init
        git add 

        .git目录存放的是本地库相关的子目录和文件, 不要删除和混乱修改
    (2)设置签名
        形式:
            用户名: tom
            Email: 123@qq.com 
            //区分不同开发人员身份
            //签名与远程库没有任何关系
        命令: 
            1.项目级别/仓库级别:
                当前本地库有效
                git config user.name tom 
                git config user.email bywww@163.com
                //信息保存到.git/config
            2.系统用户级别:登陆当前操作系统用户的范围
                git config --global user.name tom
                git config --global user.emailbywww@163.com
                // ~/gitconfig
            //就近原则 项目级别 > 系统优化级别
            //二者都没有不允许
    (3)git基本命令
    git status  状态查看 
        工作区,暂存区状态
    git add  
        将工作区新建/修改 添加到暂存区
    
    git commit filename 
        暂存区提交到本地库
        //git commit -m "My second commit, modify good.txt" good.txt
        //不会进入到vi编辑器中
    git checkout
    git rm --cached <file>...// unstage
    (4)版本前进后退
    本质:
        指针 HEAD
    版本记录 git log 
        // 简洁 git log --pretty=oneline
        //git log --oneline //后退的历史记录
        // git reflog  //所有历史记录
    版本前进后退
        索引值(推荐) git reset --hard 局部索引值
            //git reset --hard 8712b58
        ^符号 只能后退 
            git reset --hard HEAD^
            //一个^ 表示后退一步
        ~符号 只能后退
            EX. git reset --hard HEAD~3 // 后退三步
        reset命令参数:
            
            --soft  仅仅在本地库移动HEAD指针
            --mixed 本地库移动HEAD指针,重置暂存区 
            --hard  移动本地库HEAD指针,重置暂存区,重置工作区
    (5)删除文件,找回
    前提:删除前,文件存在状态提交到了本地库
    操作:git reset --hard 指针位置
        删除操作已经提交到本地库 : 指针位置执行历史纪录
        删除操作未提交到本地库  :  指针位置使用HEAD
    删除:rm
    找回:git reset --hard 索引值

    在暂存区:
        git reset --hard HEAD
    (6)比较文件 git diff 
        git diff 
            工作区和暂存区比较
        git diff 本地库历史版本 文件名
            本地库和工作区比较
    (7)分支
    多条线同时推进工程然后合并主干
    优点: 效率,各分支不相干
    分支操作:  
        创建:git branch 分支名
        查看:git branch -v
        切换:git checkout 分支名
        合并:
            1.切换到接受修改的分支上
                git checkout 分支名1 
            2.执行merge命令
                git merge 分支名2
        解决冲突:
            1.编辑文件,删除特殊符号
            2.修改文件
            3.git add 文件名
            4.git commit -m "日志" //不需要文件名
4.Git基本原理 (SHA1)
    1.hash
        加密结果固定
        输入数据确定, 输出数据确定
        不可逆
        用途:校验文件
    2.保持版本机制
        快照
    3.管理分支
        指针
5.GitHub
    本地库:
        git init
        git add
        git commit
    远程库
        git remote -v // 查看别名
        创建别名:
            git remote  add origin https://github.com/wangbyby/smtp-golang.git
    推送:  本地-->云
        git push 别名 分支名
    克隆:
        git clone 链接
        效果: 
            1.下载
            2.创建origin别名
            3.初始化
    邀请加入团队成员
        
    拉取 云-->本地
        git pull 别名 远程分支名
        pull = fetch + merge 
            1.git fetch 别名 远程分支名
            2.git merge origin/master
    解决冲突
        要点    1.不是github远程库最新版, 不能推送, 必先拉取
                2.按"分支冲突解决" 操作解决即可
    跨团队协作
        fork 本地修改,推送到远程库
        
	SSH免密登陆(只能为一个账号)
		ssh-ketgen -t rsa -C [邮箱]
