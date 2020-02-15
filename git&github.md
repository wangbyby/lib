# 解决github每次push输入账号密码
1. git remote -v 

2. git remote rm <branch> #删除原先https连接
3. git remote add <branch> ssh地址 #连接方式更改为SSh
4. git push <branch>
5. 若有错误则 git push --set-upstream origin master 即把本地分支关联到远程主分支