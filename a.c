
// /// 第一题

// struct Node
// {
//     int value;
//     struct Node *next;
// };

// void reverseLinkList(struct Node **head)
// {

//     //1. 链表为空
//     if (head == NULL)
//     {
//         return;
//     }

//     // 2. 链表不为空
//     // 每次取出一个节点, 然后头插法插入新链表

//     struct Node *index = *head;
//     for (; index != NULL;)
//     {

//         //取出节点
//         struct Node *tmp = index;
//         index = index->next;

//         //头插法插入
//         tmp->next = (*head)->next;
//         (*head)->next = tmp;
//     }
// }

// /// 第一题结束

/// 第二题开始

struct Node
{
    int value;
    struct Node *parent;
    struct Node *left;
    struct Node *right;
};

struct Node *lowestCommonAncestor(struct Node *node1, struct Node *node2)
{

    //我们需要假设node1和node2都属于二叉树

    //1.判断输入node1, node2是否为空
    //若其中一个为空则 认为 祖先为空

    if (node1 == NULL || node2 == NULL)
    {
        return NULL;
    }

    //2. 因为有父节点指针可以直接往上遍历
    struct Node *tmp1 = node1;
    struct Node *tmp2 = node2;

    //因为节点所在的高度不一样
    //故先计算出高度

    int h1 = 0, h2 = 0;
    for (; tmp1 != NULL; tmp1 = tmp1->parent)
    {
        h1++;
    }
    for (; tmp2 != NULL; tmp2 = tmp2->parent)
    {
        h2++;
    }

    //1)高度一致
    if (h1 == h2)
    {
        for (; tmp1 != NULL && tmp2 != NULL; tmp1 = tmp1->parent, tmp2 = tmp2->parent)
        {
            if (tmp1->value == tmp2->value)
            {
                return tmp1;
            }
        }
    }
    else
    {
        struct Node *higher;
        struct Node *lower;

        if (h1 > h2)
        {
            higher = tmp1;
            lower = tmp2;
        }
        else
        {
            higher = tmp2;
            lower = tmp1;
        }

        //higher先走 high-low步
        for (int i = abs(h1 - h2); i >= 0; i--)
        {
            higher = higher->parent;
        }

        for (; higher != NULL && lower != NULL; higher = tmp1->parent, lower = tmp2->parent)
        {
            if (higher->value == lower->value)
            {
                return higher;
            }
        }
    }
}

/// 第二题结束

int findLongestAscending(int *A, int n)
{
    if (n <= 1)
    {
        return n - 1;
    }

    int dp[] = new int[n]; //新建数组并初始化为0

    int maxlength = 0, maxindex = 0;

    for (int i = 1; i <= n; i++)
    {
        if (A[i] >= A[i - 1])
        {
            dp[i] = dp[i - 1] + 1;
        }

        maxlength = max(maxlength, dp[i]);
        if (maxlength == dp[i])
        {
            maxindex = i;
        }
    }
    return maxindex;
}
