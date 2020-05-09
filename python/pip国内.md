# linux

* 创建pip.conf文件
* 首先运行以下命令
        ```bash
        cd ~/.pip   # 运行此命令切换目录
        ```

* 如果提示目录不存在，自行创建一个(如果目录存在，可跳过此步)，如下：
        ```bash
        mkdir ~/.pip
        cd ~/.pip
        ```

* 在 .pip目录下创建一个 pip.conf 文件，如下：
        ```bash
        touch pip.conf
        ```
* 编辑 pip.conf 文件
* 首先打开文件，命令如下：
        ```bash
        sudo vi ~/.pip/pip.conf
        ```
* 接着，写入以下内容：
[global] 
index-url = https://pypi.tuna.tsinghua.edu.cn/simple
[install]
trusted-host = https://pypi.tuna.tsinghua.edu.cn  # trusted-host 此参数是为了避免麻烦，否则使用的时候可能会提示不受信任
然后保存退出即可。


# windows

在user目录下新建 pip的文件夹
新建pip.ini文件
写入
```ini
[global]
index-url = https://pypi.tuna.tsinghua.edu.cn/simple
[install]
trusted-host = https://pypi.tuna.tsinghua.edu.cn
# trusted-host 此参数是为了避免麻烦，否则使用的时候可能会提示不受信任
```

# 错误


>Fatal error in launcher: Unable to create process using '"c:\users\13298\anaconda3\python.exe"  "D:\coding_app\Anaconda3\Scripts\pip.exe" ': ???????????
1. 卸载``python -m pip uninstall pip``
2. 安装
   1. ``wget https://bootstrap.pypa.io/get-pip.py``
   2. ``python get-pip.py``


# 依赖

- 导出: ``pip freeze > requirements.txt``
- 导入: ``pip install -r requirements.txt``