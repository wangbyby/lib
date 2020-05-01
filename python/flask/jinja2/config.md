
## 类导入配置

0. 目录结构

```bash

C:.
│  bin.sh
│  main.py
│
├─config
│  │  config.py
│  │  db.sql
│  │  init.sql
│  │  __init__.py
│  │
│  └─__pycache__
│          config.cpython-36.pyc
│          __init__.cpython-36.pyc
│
├─controllers
├─models
│      users.py
│
└─templates
        404.html
        500.html
        base.html
        index.html
        user.html
```

1. 主模块``main.py``

```python


from config.config import Config

class NameForm(FlaskForm):
    name = StringField('What is your name?', validators=[DataRequired()])
    email = StringField('What is your email address?', validators=[DataRequired()])
    password = PasswordField('What is your password?', validators=[DataRequired()])
    submit = SubmitField('Submit')


app = Flask(__name__)
app.config.from_object(Config)


```

2. 从模块

记得``__init__.py``文件

```python

from ..models.user import ...

```
