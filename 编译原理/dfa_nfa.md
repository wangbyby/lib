
- $F$: 接受状态集
- $s_0$: 开始状态
- $move$: 转换函数

# DFA

- 模拟DFA
    - s = $s_0$
    - c = next_char()
    - while c != EOF
      - s = move[s,c]
      - c = next_char()
    - return s in F


# NFA

- 模拟NFA
  - S = $\varepsilon-closure(s_0)$
  - c= next_char()
  - while c != EOF
    - S = $\varepsilon-closure(move(S,c))$
    - c = next_char()
  - return $S \cap F != \emptyset$

## 实现模拟NFA

- 需要两个堆栈. ``oldStates``当前状态集合, ``newStates``下一个状态集合
- 以NFA状态为下标的布尔数组aleadyOn
- $move[s,a]$: NFA的转换表, 表中条目是状态集合

- addState(s)
  - newState.push(s)
  - aleadyOn[s] = true
  - for i in $move[s,\varepsilon]$
    - if !aleadyOn[t]
      - addState[t]

- ``模拟NFA的``算法
  - aleadyOn设置为全false
  - for s in $\varepsilon-closure(s_0)$
    - addState(s)
  - c  = next_char()
  - while c != EOF
    - while s=oldstate.pop()
      - for t in move[s,c]
        - if ! aleadyOn[t]
          - addState(s)
    - while s=newStates.pop()
      - oldState.push(s)
      - aleadyOn[s]=false
    c = next_char()
  return $S \cap F \neq \emptyset$

# NFA$\rightarrow$DFA

1. NFA状态集上的操作

|操作|描述|
|-|-|
|$\varepsilon-closure(s)$|从NFA的状态s开始通过$\varepsilon$到达的NFA状态集合|
|$\varepsilon-closure(T)$|$T$中某个NFA的状态s开始通过$\varepsilon$到达的NFA状态集合, 即$\cup_{s \in T}\varepsilon-closure(s)$|
|$move(T,a)$|从$T$中某个状态s出发通过标号为a转换到达NFA状态的集合|


2. 实现NFA$\rightarrow$DFA
    - 开始: Dstates = [$\varepsilon-closure(s)_0$ ]
    - while 未标记状态T in Dstates
      - 给T打上标记
      - for 每个输入符号a
        - U = $\varepsilon-closure(move(T,a))$
        - if U not in  Dstates
          - 将U加入到Dstates中, 且不加标记
        - $Dtran{T,a}=U$

3. 计算$\varepsilon-closure(T)$

   - 将T所有状态压入stack中
   - 将$\varepsilon-closure(T)$初始化为T
     - while $stack \neq \empty$
       - t = stack.pop()
       - for u in 从t出发经过$\varepsilon$到达状态u
         - if u not in $\varepsilon-closure(T)$
           - u加入到$\varepsilon-closure(T)$
           - stack.push(u)


