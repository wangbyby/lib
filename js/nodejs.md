# 数据库

```js
    var exp = require('express'); // web框架
var pg = require('pg'); //postgresql 数据库
const path = require('path'); // 路径
var bodyParser = require('body-parser');
let fs = require('fs'); //filesystem

var app = exp() ;// express web应用
// 创建 application/x-www-form-urlencoded 编码解析
var urlencodedParser = bodyParser.urlencoded({
    extended: false
});
const connString = "postgres://postgres:root@localhost:5432/postgres";
var client = new pg.Client(connString); //数据库
var server_run_place;
//初始化
// 读取配置文件并初始化数据库client
fs.readFile("hostposrt.json", function (err, data) {
    var setting = JSON.parse(data.toString()) ;//读取配置文件
    server_run_place = 'http://' + setting['host'] + ':' + setting['port'];
    client.connect(function (err) {
        if (err) {
            client.end();
            console.error("pgsql链接错误", err);
        }
        var server = app.listen(setting['port'], function () {
            // var host = server.address().address
            // var port = server.address().port
            console.log("应用实例，访问地址为" + server_run_place)
        });
    });
});


// 自定义跨域中间件
var allowCors = function (req, res, next) {
    res.header('Access-Control-Allow-Origin', req.headers.origin);
    res.header('Access-Control-Allow-Methods', 'GET,PUT,POST,DELETE,OPTIONS');
    res.header('Access-Control-Allow-Headers', 'Content-Type');
    res.header('Access-Control-Allow-Credentials', 'true');
    next();
};
app.use(allowCors); //使用跨域中间件
app.use(exp.static(path.join(__dirname, 'views'))) //导入静态文件



app.get('/', function (req, res) { //主页面
    res.sendFile(__dirname + "/" + "./views/login.htm");
})

//登录逻辑
app.post('/login', urlencodedParser, function (req, res) {
    var response = {
        "user": req.body.user,
        "password": req.body.password
    };
    res.set('Accept', 'application/json')

    console.log(response)
    var sql = "select name,password from stu where name=$1"
    client.query(sql, [response.user], function (err, result) {
        console.log("查询结果" + result.rows)
        if (result.rows.length != 0) {
            //已注册
            console.log("已注册")
            res.redirect(server_run_place)
        } else {
            var a = 'insert into stu(name,password) values ($1,$2)'
            client.query(a, [response.user, response.password], function (err, result) {
                //console.log(result)
                if (err != null) {
                    console.log("insert错误", err)
                }
            })
        }
    })
    res.status(200).json(response)
    res.end()
})


//查询数据库
/*
    /query?type=del&id=1
    /query?type=query&id=1
    /query?type=update&id=1&name=...&price=... //更新已存在用户
    /query?type=insert&id=1&name=...&price=...//新添用户

*/
//你重点要改的地方
app.get('/query', urlencodedParser, function (require, response) {
    console.log("query of params = ", require.query);
    var sql = "";
    var type = require.query.type;
    switch (type) {
        case 'del':
            sql = "delete from stu where id=$1"
            client.query(sql, [require.query.id], function (err, result) {
                if (err) {
                    console.error('query pgsql error', err)
                    response.end('del error')
                } else {
                    response.end('del OK')
                }
            })
            break;
        case 'query':
            var tuple = "id, name,password"
            sql = "select " + tuple + " from stu where id=$1"
            client.query(sql, [require.query.id], function (err, result) {
                if (err) {
                    console.error('query pgsql error', err)
                } else {
                    response.end(JSON.stringify(result.rows))
                    console.log("数据库查询结果", result.rows)
                }
            })
            break;
        case 'update':
            var values = [require.query.name, require.query.password, require.query.id]
            sql = "update stu set name=$1,password=$2 where id=$3";
            client.query(sql, values, function (err, result) {
                if (err) {
                    console.error('query pgsql error', err)
                    response.end('update error')
                } else {
                    response.end('update OK')
                }
            })
            break;
        case 'insert':
            var values = [require.query.name, require.query.password]
            sql = "insert into stu (name,password) values ($1,$2)"
            client.query(sql, values, function (err, result) {
                if (err) {
                    console.error('query pgsql error', err)
                    response.end('insert error')
                } else {
                    response.end('insert OK')
                }
            })
            break;
        default:
            response.end("ERROR")
            break;
    }
})
```