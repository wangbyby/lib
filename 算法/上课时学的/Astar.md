
# A* 算法 F = G+H

```
    Openlist, Closelist, F,G,H
    起点:S
    终点:E
    路径:P
    1. Openlist.append(S)
    2.
        while E not in Openlist || Openlist != null
            u = Openlist.Min(F) #按F排序
            Openlist.append(u)
            Closelist.append(u)
            for i in u.adj() //查找u的邻接节点
                if i in closelist || i in 不可抵达
                    pass
                else
                    if i not in Openlist
                        Openlist.append(i)
                        i.father = u
                        P.append(u) // 包含u的F, G, H
                    else
                        if i.G  < u.G + w(u,i)
                            i.father = u
                            recompute i.G, i.F, i.H
        return P
```