
def match(instr: str)->bool:
    num = [str(_) for _ in range(0,10)]
    state = 1
    for i in instr:
        if i in num and state ==1:
            state = 2
        elif state==2:
            if i in num:
                state =2
            elif i==".":
                state =3
            else:
                return False
        elif state==1 and i==".":
            state=3
        elif state== 3:
            if i in num:
                state =2
            else :
                return False
        else:
            return False
    return True
                

for i in ["111","12.12",".12","-1e10","-1.123"]:
    res = match(i)
    print(res)