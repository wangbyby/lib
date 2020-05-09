
# jvm如何寻找类?

搜索的先后顺序
1. 启动类 jre\lib
2. 扩展类 jre\lib\ext
3. 用户类 默认当前目录

- DirEntry one file
- ZipEntry zip_files
- CompositeEntry entrys
- WildcardEntry 同上

# 解析class文件


- 魔数
- 版本号
- 类/超类索引  ``java.lang.Object``是0
- 接口索引
- 字段和方法
- 常量池
  - 字面量 字符串/数字
  - 符号引用 类/接口名/字段/方法信息
- 属性
  

# 运行时数据区

1. 多线程共享
   1. java虚拟机启动时创建好
   2. java虚拟机退出时销毁
   3. 内存区域
      1. 类数据
         1. 方法区
      2. 类实例
         1. 堆

2. 线程私有
   1. 创建线程才创建
   2. 线程退出才销毁
   3. pc寄存器&java虚拟机帧栈
      1. 帧栈中含局部变量表&操作数栈


# 类
    1. 方法区
       1. 类变量

- 运行时常量池:
  - 字面量
  - 符号引用