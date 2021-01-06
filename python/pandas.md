

# pandas的使用



**注意, 修改值的函数都要考虑`inplace`是否为`True`**



```python
l = ['hello', 'wrod', 'hello']

#去重, 效率其实不太高
s = pd.Series(l).value_counts()

s.sort_values(ascending=False, inplace=True)

#python的 匿名函数语法和狗屎差不多...
df = pd.DataFrame(map(lambda x: (
    x[0]+'!!!', x[0], x[1]), s.items()), columns=['word', 'pos', 'times'])

```

# DataFrame去重

```
df = pd.read_csv(inputfile_path)

# 重复项计数
df.insert(1, 'counts', 1)
nodup = df.groupby(columns).count()
#排序
nodup.sort_values(by=['current', 'counts'], ascending=False, inplace=True)
nodup.to_csv(output)
```

2. 简单的原地去重

```python
df.drop_duplicates(inplace=True)
```



## 读取csv文件的代码



1. 效率低下的版本

```
one_fileseries = pd.Series()

for line in infileobj.readlines():

    oneline = [word.strip() for word in line.split(
    ' ') if word.strip() not in stop_words]
    line_series = pd.Series(oneline)

    one_fileseries.append(line_series)

counter = one_fileseries.value_counts()
counter.sort_values(ascending=False, inplace=True)

total = map(lambda x: (
help_remove_postag(x[0]), x[0], x[1]), counter.items())

df = pd.DataFrame(total, columns=columns)
df.to_csv(outname)
```

2. 效率还行的版本

```
counter: Dict[str, int] = {}
for line in infileobj.readlines():
	for word in line.split(' '):
		word = word.strip()
		if word in stop_words:
			continue
		t: int = counter.get(word, 0)
		counter[word] = t+1

tmp = sorted(counter.items(), key=lambda x: x[1], reverse=True)
tmp2 = map(lambda x: (help_remove_postag(x[0]), x[0], x[1]), tmp)

df = pd.DataFrame(tmp2, columns=columns)
df.to_csv(outname)
```



## 多个DataFrame写入同一个csv

```
with open('a', 'w', encoding='utf-8') as f:

        pd.DataFrame(columns=['f']).to_csv(f, index=False) # 去除index

        df1 = pd.DataFrame([1, 2, 3], columns=['f'])
        df2 = pd.DataFrame([4, 5, 6], columns=['f'])

        df1.to_csv(f, mode='a+', index=False,  header=False)
        df2.to_csv(f, mode='a+', index=False,  header=False)

df = pd.read_csv('a')
print(df)
```



## 排序



1. series

```python
l = [('a', 15), ('b', 20)]
s = pd.Series(l)
s.sort_values(ascending=False, inplace=True)
"""
1    (b, 20)
0    (a, 15)
"""

a = {'a': 12, 'b': 100}
s = pd.Series(a)

s.sort_values(ascending=False, inplace=True)
"""
b    100
a     12
"""

print(s)
```



2. DataFrame

```python
df.sort_values(by='col1', ascending=False)
```



## 与Dict交互

```python
d = {'name': ['wby', 'li'], 'age': [10, 2000]}
df = pd.DataFrame(d)
print(df)
"""
  name   age
0  wby    10
1   li  2000
"""


d = {'name': 10, 'world': 20}
tmp = pd.Series(d)
tmp.sort_values(ascending=False, inplace=True)
tmp2 = map(lambda x: ((x[0]+'!!!'), x[0], x[1]), tmp.items())
df = pd.DataFrame(tmp2, columns=columns)
print(df)
"""
       word    pos  times
0  world!!!  world     20
1   name!!!   name     10
"""
```



## index

```python
df.set_index(['pos'], inplace=True)

a = df.loc['world']
print(a)
"""
word     world!!!
times          20
Name: world, dtype: object
"""
```

## 遍历行

```python
for index, row in df.iterrows():
        print row["c1"], row["c2"]
for row in df.itertuples(index=True): #这个好像快一点
        print getattr(row, "c1"), getattr(row, "c2")
```



## 列

```python
df_ict['times']
```

### cell

```
df_stanford.at['123', 'times'] # 行, 列
```



## 转为二维数组

1. 内容转为二维数组

```
dataSet = pd.read_csv('a.csv')
print(dataSet.values.tolist())
```

2. 获取表头

```
print(dataSet.columns.values.tolist())
```



### fillna



```python
#python代码
df = pd.read_csv('a.csv')
# fillna
df.fillna(value=1, inplace=True)
print(df)

#结果如下
"""
     1    2  3   4
0  5.0  6.0  7   8
1  1.0  1.0  9  10
"""
```

**a.csv**

```txt
1,2,3,4
5,6,7,8
,,9,10
```

