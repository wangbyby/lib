
## 错误

- “KeyError: 'A secret key is required to use CSRF.'”
    - 配置中加入``SECRET_KEY= 'hard to guess'``

## 类导入配置

```bash
C:.
│  bin.bat
│  config.py
│  config.sh
│  flasky.py
│  requirements.txt
│
├─app
│  │  main.py
│  │  models.py
│  │  __init__.py
│  │
│  ├─configcongig
│  │  │  db.sql
│  │  │  init.sql 
│  │
│  ├─controllers
│  ├─main
│  │  │  errors.py
│  │  │  forms.py
│  │  │  views.py
│  │  │  __init__.py
│  │  │  
│  ├─templates
│  │      404.html
│  │      500.html
│  │      base.html
│  │      index.html
│  │      user.html

```