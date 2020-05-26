
# 错误

1. ‘const NIdentifier’ as ‘this’ argument discards qualifiers
2. 
如下解决问题
```c++
void debug() const
    {
        std::cout << "Id_name= " << name<<"\t";
    }
```