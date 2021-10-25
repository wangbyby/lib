
# 关键字

- inline: 对编译器的请求
对函数的调用改为一份函数代码副本替代

- 重载
编译器是无法根据返回值类型来判断调用语境的
原因之一是有隐式转换吗?

- assign vs. 构造函数初始化
    = : 内置类型或者单一值来初始化
    构造函数: 多值初始化

# 类

- 构造函数的奇怪之处
```c++

vector<int> v; //构造v
vector<int> v(); // 函数v返回vector, 是为了兼容c语言
```

- copy constructer 提供独立数据的副本 类似与rust的clone

- mutable 和const

const: 成员函数不改变class object的值
```c++
int length() const {return len;}
```


mutable: 对变量的改变不会破坏class object的常量性

- this 指针
成员函数中指向其调用者