
# $O(1)调度$
1. 思想
   - task的``优先级线性映射到固定时间片``
   - 根据task的优先级做时间片轮转

2. $O(1)调度具体操作$

   ``双队列,优先级数组,位图``详见PPT

3. $O(1)调度可能的问题$
   1. 很多高优先级的task, 造成低优先级task的饥饿
   2. 多个低优先级task, 频繁切换
   如何解决?
       - 问题1
          - 双斜率映射，即低优先级比较抖，高优先级比较缓。
          - 设置最大饥饿容忍时间，到时间后强行切换调度。
       - 问题2
          - 加入限制 

# CFS调度
## 思想
> CFS basically models an "ideal, precise multi-tasking CPU" on real hardware.
CPU that has 100% physical power and which can run each task at precise equal speed, in parallel, each at 1/nr_running speed. For example: if there are 2 tasks
running then it runs each at 50% physical power - totally in parallel.

<!-- 优先级与时间片解耦 -->

## 关键
1. 如何确定task的时间片
2. 如何选择下一个被调度执行的线程。


<!-- 调度器定义一个固定的时间间隔，在这个间隔期间系统中的每一个线程都至少要运行一次。
这个时间间隔按权重比例被分配到各线程。这个被分配后生成的时间间隔我们就叫作  ``timeslice``。
一个线程的$权重$本质上就是它的$优先级$，或者是UNIX系统所说的``niceness``。有更低``niceness``值的线程有更高的权重，反之亦然。

当一个线程运行时，CFS会统计其``vruntime``(该线程按其权重被分配到的运行时间)。一旦一个线程的``vruntime``超过了它被分配的时间片，如果此时有其它可运行的线程，那么当前这个线程将被从当前线程抢占。如果有最小的``vruntime``值的其它线程被唤醒，那么当前这个线程也可能被抢占。

所有的线程都被组织进用红黑树实现的``runqueue``中，且按照它们的``vruntime``递增顺序排序存储。当一个CPU是查找一个新的线程来运行时，它就直接获取这个红黑树最左侧的叶子节点，因为这个节点包含的线程有最小的``vruntime``。 -->
### 1.确定时间片
> 假设有$n$个task,  在一个固定的时间间隔$T$内.
每个线程分配到的``timeslice``$T_i:$$$T_i = T\frac{w_i}{\sum_{i=1}^{n} w_i} $$
其中$w_i$为该task的权重, $\sum_{i=1}^{n} w_i$为所有task权重之和.
那么每个线程的``vruntime``$v_i:$
$$v_i = T_i\frac{w_{base}}{ w_i} =  T\frac{w_i}{\sum_{i=1}^{n} w_i}\frac{w_{base}}{ w_i} = T\frac{w_{base}}{\sum_{i=1}^{n} w_i}$$
故:
$$v_1=v_2=...=v_n=T\frac{w_{base}}{\sum_{i=1}^{n} w_i}$$
故两次时钟tick时间差$\Delta T$之间，当前task的 `virtual runtime` 流逝必须是$\Delta T\frac{w_{base}}{\sum_{i=1}^{n} w_i}$才能保证所有task的 ``virtual runtime``趋向于一致，保证公平.

### 2.挑选下一个要运行的task
<code>挑选virtual runtime最小的task即可</code>

### 3.算法 

1. 一个task投入运行时：
   $$curr.exec\_start = now$$
2. 时钟tick时：
$$\Delta T = now- curr.exec\_strat $$$$
v_i = v_i+ \Delta T\frac{w_{base}}{\sum_{i=1}^{n} w_i}$$

   如果$\Delta T > T \times \frac{w_{base}}{\sum_{i=1}^{n} w_i}$ , 就切换到下一个task。
3. 切换时：
   将当前task以$v_i$为键值插入队列, 在队列中选取$v$最小的task投入运行


## CFS具体操作

### 0. 数据结构
```c++
/* CFS-related fields in a runqueue */
struct cfs_rq {
	struct load_weight load;//所有task权重之和
	unsigned long nr_running;//task的个数

	u64 exec_clock; // 运行的时钟
	u64 min_vruntime; // 红黑树中最小的vruntime值

	struct rb_root tasks_timeline;//红黑树的根结点
	struct rb_node *rb_leftmost;//指向vruntime值最小的结点

	struct list_head tasks;
	struct list_head *balance_iterator;

	/*
	 * 'curr' points to currently running entity on this cfs_rq.
	 * It is set to NULL otherwise (i.e when none are currently running).
	 */
	struct sched_entity *curr, *next;

	unsigned long nr_spread_over;
   // ...
   };

struct rq {
	/* runqueue lock: */
	spinlock_t lock;

   long nr_running;//task的个数
	#define CPU_LOAD_IDX_MAX 5
	unsigned long cpu_load[CPU_LOAD_IDX_MAX];
	unsigned char idle_at_tick;

	/* capture load from *all* tasks on this cpu: */
	struct load_weight load;
	unsigned long nr_load_updates;
	u64 nr_switches;

	struct cfs_rq cfs; // 非实时
	struct rt_rq rt; //实时

	unsigned long nr_uninterruptible;

	struct task_struct *curr, *idle;
	unsigned long next_balance;
	struct mm_struct *prev_mm;

	u64 clock;

	atomic_t nr_iowait;

	struct lock_class_key rq_lock_key;
};

struct sched_entity {
   /* for load-balancing 负荷权重，
   这个决定了进程在CPU上的运行时间和被调度次数*/
	struct load_weight	load;		
	struct rb_node		run_node;
	struct list_head	group_node;
	unsigned int		on_rq; // 是否在runqueue

	u64			exec_start; // 开始时间
	u64			sum_exec_runtime; //这次总的运行时间
	u64			vruntime;
	u64			prev_sum_exec_runtime; //上次运行时间

	u64			last_wakeup;
	u64			avg_overlap;
   ...
   };

```
### 1. 运行时
```c++
/*
update_curr函数功能
   1. 首先计算进程当前时间与上次启动时间的差值
   2. 通过负荷权重和当前时间模拟出进程的虚拟运行时钟
   
*/
static void update_curr(struct cfs_rq *cfs_rq)
{
	struct sched_entity *curr = cfs_rq->curr;/*  确定runqueue的当前执行进程*/
	u64 now = rq_of(cfs_rq)->clock;
	unsigned long delta_exec;

	if (unlikely(!curr))//  如果该runqueue没有进程在执行
		return;

	/*
	 * Get the amount of time the current task was running
	 * since the last time we changed load (this cannot
	 * overflow on 32 bits):
	 * 即得到本次tick实际运行的时间值
    */
	delta_exec = (unsigned long)(now - curr->exec_start);

   //将本次tick实际运行的时间值更新到vruntime和实际运行时间
	__update_curr(cfs_rq, curr, delta_exec);
	
   curr->exec_start = now;//更新开始时间, 下次计算使用

	if (entity_is_task(curr)) {
		struct task_struct *curtask = task_of(curr);

		cpuacct_charge(curtask, delta_exec);
	}
}

static inline void
__update_curr(struct cfs_rq *cfs_rq, struct sched_entity *curr,
	      unsigned long delta_exec)
{
	unsigned long delta_exec_weighted;

	schedstat_set(curr->exec_max, max((u64)delta_exec, curr->exec_max));

         //在这里计算sum_exec_runtime，它等于进程从创建开始占用CPU的总时间 
	curr->sum_exec_runtime += delta_exec; 

	schedstat_add(cfs_rq, exec_clock, delta_exec);

	delta_exec_weighted = delta_exec;
	if (unlikely(curr->load.weight != NICE_0_LOAD)) {//nice != 0
		delta_exec_weighted = calc_delta_fair(delta_exec_weighted,
							&curr->load);
	}
   //若nice==0, 虚拟时钟和物理时间相等
   
	curr->vruntime += delta_exec_weighted;
}
   ```

### 2. 选取下一个要运行的task

<img src="./aaa.png"> </img>

```c++
asmlinkage void __sched schedule(void)
   next = pick_next_task(rq, prev);

static inline struct task_struct *
pick_next_task(struct rq *rq, struct task_struct *prev)

   if (likely(rq->nr_running == rq->cfs.nr_running)) {
        p = fair_sched_class.pick_next_task(rq);
   }

static struct task_struct *pick_next_task_fair(struct rq *rq)
   do {
		se = pick_next_entity(cfs_rq); //选择下一个
		cfs_rq = group_cfs_rq(se);
	} while (cfs_rq);

static struct sched_entity *pick_next_entity(struct cfs_rq *cfs_rq)
    if (first_fair(cfs_rq)) {
		se = __pick_next_entity(cfs_rq);
		se = pick_next(cfs_rq, se);
		set_next_entity(cfs_rq, se);
	}


static struct sched_entity *__pick_next_entity(struct cfs_rq *cfs_rq)
    return rb_entry(first_fair(cfs_rq), struct sched_entity, run_node);

static inline struct rb_node *first_fair(struct cfs_rq *cfs_rq)
{
	return cfs_rq->rb_leftmost;
}

```
## CFS的几个疑问
- 新进程的``vruntime``值是多少?
   不会初始化为0, 否则老进程们都饿死了.
   所以说应该是一个接近``min_vruntime``的值
- 休眠进程的``vruntime``一直保持不变?
   同上, 不会.
   在休眠进程被唤醒时会重新设置vruntime值，以min_vruntime值为基础，减去调度周期的一半
- 休眠进程在唤醒时会立刻抢占CPU吗?
   大概率会, 这也是CFS调度算法的本意，即保证交互式进程的响应速度，因为交互式进程等待用户输入会频繁休眠
- task占用的CPU时间片可以无穷小吗?
   为了避免过于短暂的进程切换造成太大的消耗，CFS设定了进程占用CPU的最小时间值，sched_min_granularity_ns，正在CPU上运行的进程如果不足这个时间是不可被调离CPU

- 不足
   多核问题,详见[The Linux Scheduler: a Decade of Wasted Cores
](http://ece.ubc.ca/~sasha/papers/eurosys16-final29.pdf)

# 附录
- [The Linux Scheduler: a Decade of Wasted Cores
](http://ece.ubc.ca/~sasha/papers/eurosys16-final29.pdf)
- [Linux2.6代码下载](https://mirrors.edge.kernel.org/pub/linux/kernel/v2.6/)
- [参考1](https://blog.csdn.net/dog250/article/details/96012688?)
- [参考2](https://blog.csdn.net/weixin_33725239/article/details/94303653)
- [参考3](https://www.cnblogs.com/lh03061238/p/12297214.html)