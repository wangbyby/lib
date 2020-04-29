
计数: while循环

# 1. 数据结构的存储方式
- 数组(连续)
- 链表(链式)

# 2. 基本操作
**遍历+访问**

遍历
  - 迭代: 数组, 链表
  - 递归: 树(前,中,后序), 链表


## 动态规划
暴力解+cache

## 回溯
```python
result = []
def backtrack(路径, 选择列表):
    if 满足结束条件:
        result.add(路径)
        return
    
    for 选择 in 选择列表:
        做选择
        backtrack(路径, 选择列表)
        撤销选择
```
## 二分查找

``left, right, mid ,target, index``

``区间: [left,right]``

```python
fn search(nums,target){
    left = 0;
    right = length-1;
    while left <= right{
        mid = left+(left-right)/2 //会不会溢出呢?
        if nums[mid] == target {
            ...
        }else if nums[mid] > target{
            
            [if ]
            
            right = ...
        }else if nums[mid] < target{
            [if ]
            left = ...
        }
    }
    return ...
}
```

Ex.
```python
class Solution:
    def firstBadVersion(self, n):
        """
        :type n: int
        :rtype: int
        """
        left = 1
        right = n-1
        
        mid = left
        while left <=right:
            
            mid = int((left+right)/2)
            
            if isBadVersion(mid):
                
                right = mid-1
            else:
                left = mid+1
        if not isBadVersion(right) and  isBadVersion(right+1):
            return right+1
        return mid

# //给定一个排序好的数组，两个整数 k 和 x，从数组中找到最靠近 x（两数之差最小）的 k 个数。返回的结果必须要是按升序排好的。如果有两个数与 x 的差值一样，优先选择数值较小的那个数。
# 说明:

#     k 的值为正数，且总是小于给定排序数组的长度。
#     数组不为空，且长度不超过 10^4
#     数组里的每个元素与 x 的绝对值不超过 10^4

class Solution:
    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:
     
        left = 0
        
        right  = len(arr) - 1 - k
        while left <= right:
            mid = int((left+right)/2)
            
            if  x - arr[mid] >arr[mid+k]-x:
                left = mid+1  
            else:
                right = mid-1
        return arr[left:left+k]
# 把 k看作1
```
## 双指针

### 快慢
    slow
    fast = 2*slow
### 双下标

## 滑动窗口
