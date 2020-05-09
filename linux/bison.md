# 问题

1. 先在``xx.y``文件里添加token类型, 并且记得声明结合性``%left``, ``%right``, ``%nonassoc``
2. 文件后缀**与c++相关**

``重复一下...``

```bash
ast.hh
lex.ll
parser.yy
```

.
├── Makefile
├── a.out
├── ast.hh
├── gt.lex.c
├── gt.ll
├── gt.yy
├── gt.yy.tab.c
├── gt.yy.tab.h
├── main.cpp
└── tests
    └── t1

```bash
# Makefile
#不要瞎按 tab
a.out: gt.ll gt.yy ast.hh
        bison -d gt.yy
        flex -o gt.lex.c gt.ll
        g++ -o $@ gt.yy.tab.c gt.lex.c main.cpp

test:
        ./a.out ./tests/t1

clean:
        rm ./a.out gt.lex.c gt.yy.tab.c gt.yy.tab.h
```

