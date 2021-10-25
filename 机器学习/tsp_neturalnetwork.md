TSP旅行商问题

- 暴力方法 要死了
- dp+二进制 $n^2 2^n$起步
- 遗传算法, 粒子群算法等
  >也能处理流水线调度哦
  >对于指令之间的依赖关系, 我设想在遗传算法的变异后加一个`出生`过程, 不符合依赖关系的死掉
  >如果是粒子群算法的话就暂时不清楚啦
- 神经网络

[A_Recurrent_Neural_Network_to_Traveling_Salesman_Problem](http://www.exatas.ufpr.br/portal/docs_degraf/paulo/A_Recurrent_Neural_Network_to_Traveling_Salesman_Problem.pdf)

>The  main  types  of  Neural  Network  used  to  solve  the  Traveling  Salesman  Problem  are:  
Hopfield’s  Recurrent  Networks  (Wang  et  al.,  2002)  and  Kohonen’s  Self-Organizing  Maps  
(Leung  et  al.,  2003).  In  a  Hopfield’s  Network,  the  main  idea  is  to  automatically  find  a  
solution  for  the  Traveling  Salesman  Problem  by  means  of  an  equilibrium  state  of  the  
equation system defined for the Traveling Salesman Problem. By using Kohonen’s Maps for 
the Traveling Salesman Problem, the final route is determined through the cities 
corresponding to those neurons that have weights that are closest to the pair of coordinates 
ascribed to each city in the problem.  

- Wang: winner takes all

```c
1. 先算出AP算法的结果x
2. while w(x)在范围内{//迭代计算
3. 
    for m=0;m<n;m++{
        for k=0;k<n;k++{
            k,l = 找到第m大的元素            
            _x[k][l] = (sum(x[..][l])+sum(x[k][..]))/2
            行k, 列l其余置零
            x = _x
        }
    }
}
```