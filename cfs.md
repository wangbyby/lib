不考虑实时进程

# $O(1)调度思想$
- task的``优先级线性映射到固定时间片``
- 根据task的优先级做时间片轮转
## $O(1)调度具体操作$

``双队列,优先级数组,位图``详见PPT

## $O(1)调度可能的问题$
1. 很多高优先级的task, 造成低优先级task的饥饿
2. 多个低优先级task, 频繁切换
如何解决?
    - 问题1
       - 双斜率映射，即低优先级比较抖，高优先级比较缓。
       - 设置最大饥饿容忍时间，到时间后强行切换调度。
    - 问题2
       - 加入限制 

# CFS思想

CFS basically
models an "ideal, precise multi-tasking CPU" on real hardware.

**关键**:
1. 如何确定线程的时间片
2. 如何选择下一个被调度执行的线程。


调度器定义一个固定的时间间隔，在这个间隔期间系统中的每一个线程都至少要运行一次。
这个时间间隔按权重比例被分配到各线程。这个被分配后生成的时间间隔我们就叫作  $timeslice$。
一个线程的$权重$本质上就是它的$优先级$，或者是UNIX系统所说的``niceness``。有更低``niceness``值的线程有更高的权重，反之亦然。

当一个线程运行时，CFS会统计其``vruntime``(该线程按其权重被分配到的运行时间)。一旦一个线程的``vruntime``超过了它被分配的时间片，如果此时有其它可运行的线程，那么当前这个线程将被从当前线程抢占。如果有最小的``vruntime``值的其它线程被唤醒，那么当前这个线程也可能被抢占。

所有的线程都被组织进用红黑树实现的``runqueue``中，且按照它们的``vruntime``递增顺序排序存储。当一个CPU是查找一个新的线程来运行时，它就直接获取这个红黑树最左侧的叶子节点，因为这个节点包含的线程有最小的``vruntime``。

## CFS具体操作


## CFS的问题
CFS在多核系统上过于复杂,详见[The Linux Scheduler: a Decade of Wasted Cores
](http://ece.ubc.ca/~sasha/papers/eurosys16-final29.pdf)

[参考1](https://blog.csdn.net/dog250/article/details/96012688?)