# json序列化与反序列化

```go
    import (
        "fmt"
        "encoding/json"
    )
```

## 结构体

```go
    type Stu struct{
        Name string  `json:"name"`//反射机制
        Age int	 `json:"Age"`
        Birth string 
        Sal float64
        Skill string
    }
    //结构体序列化
    func testStruct()  {
        stu := Stu{
            Name:"皮皮虾",
            Age : 300,
            Birth:"2001-01-10",
            Sal : 3000.1,
            Skill : "喷火",
        }
        data, err := json.Marshal(&stu) //主要的方法
        if err != nil{
            fmt.Println("序列化错误",err)
        }
        fmt.Printf("struct %v\n",string(data))
    }
    //反序列化 json字符串 成 struct
    func unMarshalStruct(){
        str := `{"name":"皮皮虾","Age":300,"Birth":"2001-01-10","Sal":3000.1,"Skill":"喷火"}`
        var stu Stu
        err := json.Unmarshal([]byte(str), &stu)
        if err != nil{
            fmt.Println("err=",err)
        }
        fmt.Println("反序列化后：",stu)
    }
```

## map

```go
//map 序列化
    func testMap(){
        var a map[string]interface{}
        a = make(map[string]interface{})
        a["name"] = "红孩儿"
        a["age"] = 30
        a["address"] = [2]string{"夏威夷","墨西哥"}
        data, err := json.Marshal(a)
        if err != nil{
            fmt.Println("序列化错误",err)
        }
        fmt.Printf("map: %v\n",string(data))
    }
    // json-->map
    func unMarshalMap() {
        str := `{"name":"皮皮虾","Age":300,"Birth":"2001-01-10","Sal":3000.1,"Skill":"喷火"}`
        var a map[string]interface{}
        //反序列化时，自动make（即make 操作被封装到 unmarshal中）
        err := json.Unmarshal([]byte(str), &a)
        if err != nil{
            fmt.Println("err=",err)
        }
        fmt.Println("反序列化后为map：",a)
    }
```

## slice

```go
    //切片序列化
    func testSlice()  {
        var slice []map[string]interface{}
        var m1, m2 map[string]interface{}
        //使用map前，要make
        m1 = make(map[string]interface{})
        m1["name"] = "jack"
        m1["age"] = "7"
        slice = append(slice, m1)

        m2 = make(map[string]interface{})
        m2["name"] = "tom~~"
        m2["age"] = "1000"
        slice = append(slice, m2)

        data, err := json.Marshal(slice)
        if err != nil{
            fmt.Println("序列化错误",err)
        }
        fmt.Printf("slice: %v\n",string(data))
    }
    //json -->slice
    func unMarshalSlice() {
        str := `[{"age":"7","name":"jack"},{"age":"1000","name":"tom~~"}]`
        var slice []map[string]interface{}

        err := json.Unmarshal([]byte(str), &slice)
        if err != nil{
            fmt.Println("err=",err)
        }
        fmt.Println("反序列化后为slice：",slice)
    }
```

## 基本数据类型

```go

    //基本数据类型序列化, 意义不大
    func testFloat64(){
        var num float64 = 3.123
        data, err := json.Marshal(num)
        if err != nil{
            fmt.Println("序列化错误",err)
        }
        fmt.Printf("普通类型: %v\n",string(data))
    }
    func main()  {
        //结构体， map， 切片进行序列化
        testStruct()
        testMap()
        testSlice()
        //基本数据类型序列化
        testFloat64()
    }
```
