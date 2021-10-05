- 文件中出现最多次数的IP

`cat test.txt | awk '{print $2}' | sort | uniq -c | sort -n -r | head -n 1`

- 按规则输出文件。考seq

- 当前占据cpu，内存最多的进程

`ps -aux | sort -k4nr | head -N`
ps：参数a指代all——所有的进程，u指代userid——执行该进程的用户id，x指代显示所有程序，不以终端机来区分
sort -k4nr中（k代表从根据哪一个关键词排序，后面的数字4表示按照第四列排序；n指代numberic sort，根据其数值排序；r指代reverse，这里是指反向比较结果，输出时默认从小到大，反向后从大到小。）
    -k3表示按照cpu占用率排序

- 排查僵尸，孤儿进程
kill 父进程时一同杀死子进程
kill -9 `ps -ef |grep PID|awk '{print $2}' `


- git 想把 很多次提交移到另一分支 不用merge 
  rebase -i + cherry-pick

- 查看本网络IP
    ifconfig
    lsof -i:port
    netstat -aptn

- 找大小超过10m文件
`find /dir -size +10M`

- 打印xx文件第100行
`sed -n '1,1p' test_email.py`