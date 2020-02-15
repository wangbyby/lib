/*
    *修改于2019.12.8 JavaScript代码实现
    *动态规划思想
        1.旅行商问题
        2.广义背包问题
        3.流水线调度问题
*/

/*
旅行商问题 TSP
//D 为二维数组(nxn矩阵)
*/
function TSP(D) { //起点为0
    const INF = 10000 //定义的最大值
    var n = D.length // n的个数
    var i, j, k, min, tmp;
    var b = 1 << (n - 1); // 点集状态总数
    var dp = new Array(n) // 记录状态
    var bridge = {} //记录中间节点
    for (i = 0; i < n; i++) { //初始化dp与bridge
        dp[i] = new Array(b)
        bridge[i] = {}
        for (j = 0; j < b; j++) {
            dp[i][j] = 0
            bridge[i][j] = INF
        }
    }
    for (i = 0; i < n; i++) { //初始化 dp的第0列
        dp[i][0] = D[i][0]
    }
    //init end

    //遍历二维数组即遍历dp
    for (i = 1; i < b - 1; i++) {
        for (j = 1; j < n; j++) {
            if ((1 << (j - 1) & i) == 0) {
                //点j未访问
                min = INF
                for (k = 1; k < n; k++) { //遍历点集
                    if (1 << (k - 1) & i) {
                        //点k在集合中

                        // 松弛操作
                        tmp = D[j][k] + dp[k][i - (1 << (k - 1))]
                        if (tmp < min) {
                            min = tmp
                            dp[j][i] = min
                            bridge[j][i] = k
                        }
                    }
                }
            }
        }
    }
    //处理最后一列
    min = INF
    for (k = 1; k < n; k++) {
        // 松弛操作
        tmp = D[0][k] + dp[k][b - 1 - (1 << (k - 1))] //b-1-(1<<(k-1)) :  去掉k节点
        if (tmp < min) {
            min = tmp
            dp[0][b - 1] = min
            bridge[0][b - 1] = k
        }
    }
    var mincost = dp[0][b - 1]
    var path = [0]
    for (i = b - 1, j = 0; i > 0;) {
        j = bridge[j][i] // 下一个节点
        i = i - (1 << (j - 1))
        path.push(j)
    }
    path.push(0)
    //返回值说明 path为路径, mincost为最短花费
    return [path, mincost]
}


//TSP的测试数据
var m = [
    [0, 7, 6, 10, 13],
    [7, 0, 7, 10, 10],
    [6, 7, 0, 5, 9],
    [10, 10, 5, 0, 6],
    [13, 10, 9, 6, 0]
]


var res = TSP(m)
console.log("最短路径 : ", res[0])
console.log("最少花费 : ", res[1])

//这里程序会崩溃
var n = 23
var m = new Array(n)
for (var i = 0; i < n; i++) {
    m[i] = new Array(n).fill(0)
}
var s1 = new Date().getTime()
res = TSP(m)
var s2 = new Date().getTime()
console.log("花费时间(单位:ms):", s2 - s1)


/*
    *第一种伪代码
    for i = 1 to n
        for j = 0 to M
             for x = 0;x * w[i] <= j;x++
                m[i][j] = max(
                    m[i - 1][j],
                    m[i - 1][j - x * w[i]] + x * v[i]
                )
    *第二种伪代码
    for i=1 to n 
        for j = 0 to M
            let count = j/m[i] //向量 m与v 下标 1 to n
            for k=0 to count
                dp[i][j] = dp[i-1][j]
                let tmp = dp[i - 1][j - k * m[i]] + k * v[i]
                if dp[i][j] < tmp
                    dp[i][j] = tmp
                    path[(i,j)] = 1
    let j = M
    let i = n
    let x  be a new Vector// 下标范围 [1,n]
    while i>0 && j>0
        if path[(i,j)] ==1
            x[i]++
            j = j- m[i]
        else 
            i = i-1
    return dp[n][M] , x    
    *两种的思路一样,实现方式有一些差别
    
    *时间复杂度: O(n*M*M/(min(m)))
    *空间复杂度 : O(nM)
        
    */

/*
        M:背包最大承重量
        n:物品种类个数
        m:物品质量
    */
function GKP0(M, m, v) {
    var n = m.length
    var i, j, k;
    var dp = new Array(n + 1)
    for (i = 0; i <= n; i++) {
        dp[i] = new Array(M + 1).fill(0)
    }
    var x = new Array(n + 1).fill(0)
    var path = {}
    for (i = 1; i <= n; i++) {
        for (j = 0; j <= M; j++) {
            var count = j / m[i - 1]
            for (k = 0; k <= count; k++) {
                dp[i][j] = dp[i - 1][j]
                var tmp = dp[i - 1][j - k * m[i - 1]] + k * v[i - 1]
                if (dp[i][j] < tmp) {
                    dp[i][j] = tmp
                    path[[i, j]] = 1
                }
            }
        }
    }
    //解路径
    j = M
    i = n
    while (i > 0 && j > 0) {
        if (path[[i, j]] == 1) {
            x[i - 1]++
            j = j - m[i - 1]
        } else {
            i = i - 1
        }
    }
    x.pop()
    console.log("解=", x)
    return [dp[n][M], x]
}
/*
        M:背包最大承重量
        n:物品种类个数
        m:物品质量
    *1.
        *时间复杂度 : O(nM)
        *空间复杂度: O(nM)
        *主要循环    for (i = 1; i <= n; i++) {
                for (j =0; j <= M; j++) {
                        dp[i][j] = dp[i-1][j]
                        if(j >= m[i-1] && dp[i][j] <dp[i][j-m[i-1]]+v[i-1]) {
                            dp[i][j] = dp[i][j-m[i-1]]+v[i-1]
                            path[[i,j]] = 1
                        }
                }
            }
    */
function GKP(M, m, v) {
    var n = m.length
    
    var i, j;
    var x = {} //解向量
    // var dp = new Array(n + 1) //dp为 (n+1)*(M+1)矩阵
    for (i = 0; i <= n; i++) {
        // dp[i] = new Array(M + 1).fill(0)
        x[i] = 0
    }
    var path = new Map()

    let f = new Array(M + 1).fill(0)
    for (i = 1; i <= n; i++) {
        /*
        时间复杂度 : O(nM)
        空间复杂度: O(M)
        */
        for (j = m[i - 1]; j <= M; j++) {
            var tmp = f[j - m[i - 1]] + v[i - 1]
            if (f[j] < tmp) {
                f[j] = tmp
                path[[i, j]] = 1
            }
        }
    }
    //解路径
    j = M
    i = n
    while (i > 0 && j > 0) {
        if (path[[i, j]] == 1) {
            x[i - 1]++
            j = j - m[i - 1]
        } else {
            i = i - 1
        }
    }
    console.log("最少花费 = ", f[M])
    console.log("解向量 = ", x)

    return [f[M], x]
}

//广义背包测试数据
var M = 10
var m = [1, 6, 4, 3]
var v = [1, 3, 2, 6]
console.log(GKP(M, m, v))

M = 230
m = [20, 25, 40, 12, 31]
v = [5, 2, 3, 1, 5]
console.log(GKP(M, m, v))


GKP0(M, m, v)
//0-1背包问题
function zero_one_KP(M, m, v) {
    var n = m.length
    /*
        M:背包最大承重量
        n:物品种类个数
        m:物品质量
        时间复杂度: O(nM)
        空间复杂度: O(M)
    */
    var i, j;
    // var dp = new Array(n + 1)
    // for (i = 0; i <= n; i++) {
    //     dp[i] = new Array(M + 1).fill(0)
    // }
    let path = new Map()
    let f = new Array(M + 1).fill(0)
    for (i = 1; i <= n; i++) {
        for (j = M; j >= m[i - 1]; j--) { //从大到小
            var tmp = f[j - m[i - 1]] + v[i - 1]
            if (f[j] < tmp) {
                f[j] = tmp
                path[[i, j]] = 1
            }
        }
    }
    j = M
    i = n
    var x = new Array(n).fill(0)
    while (i > 0 && j > 0) {
        if (path[[i, j]] == 1) {
            x[i - 1] = 1
            j = j - m[i - 1]
        }
        i = i - 1
    }

    return [f[M], x]
}

// /*伪代码
// M //最大质量
// m //质量数组 下标 1 to n
// v //价值数组 下标 1 to n 
// 时间复杂度 : O(nM)
// 空间复杂度: O(M)
// */
// zero_one_KP(M,m,v) //优化后 sudo code
//     //init
//     let n = m.length
//     let dp =[1,M] //向量[1,M]
//     let path = Map //哈希表
//     //init end
//     //动态规划
//     for i=1 to n //递增
//         for j = M to m[i] //递减
//             let tmp = dp[j-m[i]]+v[i]
//             if dp[j] < tmp
//                 dp[j] = tmp
//                 path[(i,j)] = 1
//     //回溯解
//     let j = M
//     let i = n
//     let x  //解向量 下标 1 to n, 全部初始化为0
//     while j>0&&i>0
//         if path[(i,j)] ==1
//             x[i] = 1
//             j = j - m[i]
//         i = i-1
//     return dp[n+1][M], x


/*
双机流水调度问题

*/
function Ele() {
    this.key;
    this.index;
    this.job;
}

function NewEle(key, index, job) {
    var e = new Ele();
    e.key = key;
    e.index = index;
    e.job = job;
    return e
}

function flowShop(a, b) {
    var n = a.length
    var c = new Array(n)
    var eArr = new Array(n)
    for (var i = 0; i < n; i++) {
        var key = a[i] > b[i] ? b[i] : a[i]
        var job = a[i] <= b[i]
        eArr[i] = NewEle(key, i, job)
    }
    eArr.sort(function (a, b) {
        return a.key - b.key
    })

    var j = 0;
    var k = n - 1;
    for (var i = 0; i < n; i++) {
        if (eArr[i].job) {
            c[j] = eArr[i].index
            j++
        } else {
            c[k] = eArr[i].index
            k--
        }
    }
    j = a[c[0]]
    k = j + b[c[0]]
    for (var i = 1; i < n; i++) {
        j += a[c[i]]
        if (j < k) {
            k += b[c[i]]
        } else {
            k = j + b[c[i]]
        }
    }
    return [k, c]
}