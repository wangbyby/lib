
1. 连接

- hello [google](https://www.google.com)

- <https://www.google.com>

2. 段落

>


3. **希腊字母与公式**
- 公式: $ $
- 上下标
    ``上标: ^``
    ``下标: _``
    $x_{i_1}^{20}$
- 字母表示
    - $\Sigma$
    - $\lambda$
    - $\pi$
    - $\epsilon$ 与$\varepsilon$

- 集合运算
    - $\cup$ ∪
    - $\cap$ ∩
    - $\in$ ∈
    - $\subset$ 真子集
    - $\subseteq$ 子集
    - $\emptyset$ 空集
    - $\forall$ 任意
    - 
- 函数式：$$f(x)=\frac{P(x)}{Q(x)}$$
- 下划线
$\_$



- 表格

|  表头   | 表头  |
|  ----  | ----  |
| 单元格  | 单元格 |
| 单元格  | 单元格 |

``-:``右对齐
``:-:``居中对齐
``:-``左对齐

| 符号 | 含义 |
| :----  | :----: |
| a ::= b | a定义为b |

- |

$\mid$ : |

- 代码段
<code >$\rightarrow$ </code>

- 流程图

```mermaid
graph TD
   A --> B
```
```mermaid
graph LR
   A --> B
```

>
    id + [文字描述]矩形
    id + (文字描述)圆角矩形
    id + >文字描述]不对称的矩形
    id + {文字描述}菱形
    id + ((文字描述))圆形

```mermaid
graph TD
    id[带文本的矩形]
    
    id4(带文本的圆角矩形)
    id3>带文本的不对称的矩形]
    id1{带文本的菱形}
    style id1 fill:#FFF,stroke:#333,stroke-width:1px,fill-opacity:0.5
    id2((带文本的圆形))

```

```mermaid
graph LR
    start[开始] --> input[输入A,B,C]
    input --> conditionA{A是否大于B}
    conditionA -- YES --> conditionC{A是否大于C}
    conditionA -- NO --> conditionB{B是否大于C}
    conditionC -- YES --> printA[输出A]
    conditionC -- NO --> printC[输出C]
    conditionB -- YES --> printB[输出B]
    conditionB -- NO --> printC[输出C]
    printA --> stop[结束]
    printC --> stop
    printB --> stop
```

- 插入图片
<img src="./aaa.png" >函数调用关系 </img>
