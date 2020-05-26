

# 段错误 核心以转存

1. 空指针

```c++

class NVariableDeclaration : public NStatement
{
public:
    const NIdentifier &type;
    NIdentifier &id;
    NExpression *assignmentExpr; //就是他
    NVariableDeclaration(const NIdentifier &type, NIdentifier &id) : type(type), id(id) {}
    NVariableDeclaration(const NIdentifier &type, NIdentifier &id, NExpression *assignmentExpr) : type(type), id(id), assignmentExpr(assignmentExpr) {
        
    }

    void debug()
    {
        std::cout << "\n\tVariable declaration type= " << type.name <<"\t";
        id.debug();
        if(assignmentExpr!= NULL){
            assignmentExpr->debug(); //还真是这里的问题... 我需要rust的Option的Debug
        }
        std::cout <<std::endl;
    }
};

```