# cargo配置文件如下 

``.cargo/config``
```bash

[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

```
清华nb

# cargo 更新库

``cargo update -p time``