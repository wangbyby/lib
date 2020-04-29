
# 前情提要

``SLR(1)的FOLLOW集过于宽泛, 在不同境况下, 归约需要的是后面直接跟的,而不是所有的FOLLOW集, 即对与归约来说, FOLLOW集过于宽泛, 包含了不必要的信息``

- $A代表非终结符$
- $a代表变量$

- 项目集闭包
  - 内核项: 初始项$S' \rightarrow .S$以及``点不在``最左端的所有项
  - 非内核项: 除了初始项$S' \rightarrow .S$以外``点在``最左端的所有项


###  计算First集

  1. 算法开始 , 令$i=1$
  2. 判断X类型
     1. X是终结符, First(X)=X
     2. X是非终结符且 $X \rightarrow Y_1Y_2...Y_n, n \geq1$ 
        1. if $\varepsilon \notin First(Y_i)$
           - $First(X) = First(X) \cup  First(Y_i)$
        2. if $\varepsilon \in First(Y_i)$
           -  $First(X) = First(X) \cup \lbrace First(Y_i)- \lbrace \varepsilon \rbrace \rbrace$
           -  $i=i+1$
           - goto 算法开始
     3. $X \rightarrow \varepsilon$, 将$\varepsilon$加入到First(X)中 
  3. 算法结束

### 计算Follow集

1. 将``#``放入Follow(S)中, S为文法的开始符号, ``#``为结束符
2. 若存在$A \rightarrow \alpha B \beta$, 那么$Follow(B) = First(\beta) - \lbrace \varepsilon \rbrace$
3. 若$A \rightarrow \alpha B$或者 ($A \rightarrow \alpha B \beta$且$\varepsilon \in First(\beta)$) , 那么$Follow(B) = Follow(B) \cup Follow(A)$
---
- 例子
    1. $S \rightarrow AB$
    2. $A \rightarrow \varepsilon|a$
    3. $B \rightarrow \varepsilon| b$
    - First(S) = $\lbrace a,b, \varepsilon \rbrace$ 


# LR(0)


- LR0_CLOSURE(I)
   - LR0_CLOSURE(I)->Set_Items
      - J = I;
      - repeat
        - for $ A\rightarrow \alpha . B \beta \in J$ 
          - for $B \rightarrow .\gamma in G$
            - if $B \rightarrow .\gamma $ not in J 
              - 将$B \rightarrow .\gamma $加入J中
      - until 在某一轮中没有新的项加入到**J**中

- LR0_Items(G')
  - LR0_Items(G')
    - C = {LR0_CLOSURE({[$S' \rightarrow .S$]})}
    - repeat
      - for I in C
        - for 每个文法符号X
          - if $GOTO(I,X) \neq \emptyset $且不在C中
            - GOTO(I,X)加入C中
    -  until 在某一轮中没有新的项加入到**C**中

- LR(0)分析表的构建
  - 构造文法G'的项目集规范族$C = \lbrace I_0,I_1,I_2..., I_n\rbrace$
     - if $[A\rightarrow \alpha .a \beta] \in I_i$并且 $GOTO(i,a)=j$, 则Action[i,a]=``sj``
     - if $[S\rightarrow S'.] \in I_i$, 则Action[i,``#``]=Acc
     - if $[A\rightarrow \alpha.] \in I_i$, 则**for $a \in T$**, 令Action[i,a]=r$A\rightarrow \alpha.$
     - if $GOTO(I_i,A)=I_j$, 令GOTO[i,A]=j

# SLR(1)


- SLR(1)分析表的构建
  - 构造文法G'的项目集规范族$C = \lbrace I_0,I_1,I_2..., I_n\rbrace$
     - if $[A\rightarrow \alpha .a \beta] \in I_i$并且 $GOTO(i,a)=j$, 则Action[i,a]=``sj``
     - if $[S\rightarrow S'.] \in I_i$, 则Action[i,``#``]=Acc
     - if $[A\rightarrow \alpha.] \in I_i$, 则**for $a \in Follow(A)$**, 令Action[i,a]=r$A\rightarrow \alpha.$
     - if $GOTO(I_i,A)=I_j$, 令GOTO[i,A]=j
``其余与LR(0)相同``

# LR(1)

- LR1_CLOSURE(I)
  - repeat
    - for $\forall[A \rightarrow \alpha.B \beta, a] \in I$
      - for $\forall [B\rightarrow \gamma] \in G'$
        - for $\forall b \in First(\beta a) $
          - 将$[B \rightarrow .\gamma,b ]$加入集合I中
  - until``I``中不能加入更多的项
  - return I

- LR1_GOTO(I,X)
  - J = {}
  -  for $\forall [A \rightarrow \alpha.X \beta, a] \in J$
       - 将$[A \rightarrow \alpha X.\beta, a]$加入集合J中
  - return LR1_CLOSURE(J)

- items(G')
  - C = LR1_CLOSURE( { $[S'\rightarrow .S,\sharp]$})
  - repeat
    - for $\forall I \in C$
      - for 每个文法符号X
        - if $GOTO(I,X) \neq \emptyset $且不在C中
          - GOTO(I,X)加入C中
  - until 不再有新的项加入到``C``中

- LR(1)分析表的构建
  - 构造文法G'的LR(1)项目集规范族$C' = \lbrace I_0,I_1,I_2..., I_n\rbrace$
     - if $[A\rightarrow \alpha .a \beta, b] \in I_i$并且 $GOTO(i,a)=j$, 则Action[i,a]=``sj``
     - if $[S\rightarrow S'., \sharp] \in I_i$, 则Action[i,``#``]=Acc
     - if $[A\rightarrow \alpha., a] \in I_i$, 则令Action[i,a]=r$A\rightarrow \alpha.$
     - if $GOTO(I_i,A)=I_j$, 令GOTO[i,A]=j


# LALR(1)

## 传播法

- 确定向前看符号:``look_ahead(K,X)``
  - 输入: 项目集I的内核**K**, 文法符号**X**
  - 算法描述
    - for $\forall [A \rightarrow \alpha.\beta] \in K$
      - J = LR1_CLOSURE({[$A \rightarrow \alpha.\beta, \sharp$]})
      - if $[B \rightarrow \gamma.X \delta, a ] \in J$
        - 则$[B \rightarrow \gamma X. \delta, a ] \in GOTO(I,X)$中的``a``是自发生成
        - //build 自发生成表
      - if $[B \rightarrow \gamma.X \delta, \sharp ] \in J$
        - 向前看符号从I中$A \rightarrow \alpha.\beta$传播到GOTO(I,X)中的项$B \rightarrow \gamma X. \delta$之上
        - //build 传播表
  - 输出: 自发生成表, 传播表


- 计算LALR(1)项目集族内核
  - let C = G的LR(0)项目集族 , 并删除其中的非内核项
  - let table = {}; let broadcast = {}
  - for $\forall X \in V_N \cup V_T$
    - for $\forall K \in C$
      - let tmp_table, tmp_broadcast = look_ahead(K,X)
      - table.extend(tmp_table);
      - broadcast.extend(tmp_broadcast);
  - 传播
    - for $\forall (item_i, item_js ) \in broadcast$
      - for $\forall item_j \in item_js$
        - 把$item_i$的展望符传播到$item_j$中去


- LALR(1)分析表的构建
  - 构造文法G'的LALR(1)项目集规范族$C' = \lbrace I_0,I_1,I_2..., I_n\rbrace$
     - if $[A\rightarrow \alpha .a \beta, b] \in I_i$并且 $GOTO(i,a)=j$, 则Action[i,a]=``sj``
     - if $[S\rightarrow S'., \sharp] \in I_i$, 则Action[i,``#``]=Acc
     - if $[A\rightarrow \alpha., as] \in I_i$, 则for $\forall a \in as$: then  Action[i,a]=r$A\rightarrow \alpha.$
       - //``as``是展望符的集合
     - if $GOTO(I_i,A)=I_j$, 令GOTO[i,A]=j
