

```bash
# 开启服务
cd C:\postgresql-10.9-1-windows-x64-binaries\pgsql\bin

pg_ctl -D data -l logfile start

C:\redis\redis-server.exe
``

- 打开redis
	redisPtr, errg = redis.Dial("tcp", "127.0.0.1:6379") //向本机链接
	if errg != nil {
		panic(errg)
	}

- 打开pgsql
	//用户是postgres 密码 root
pgDB, errg = sql.Open("postgres", "host=localhost port=5432 user=postgres password=root dbname=postgres sslmode=disable")
	if errg != nil {
		panic(errg)
	}


- sql查找一行
	sqlSta := `select "name","password" from public.stu where id=1`
	row := pgDB.QueryRow(sqlSta)
	row.Scan(&auser.User, &auser.Password)

- sql 查找很多行
		mm := make(map[string]string)
		sqlcontent := `select  "name","password" from public.stu `
		row, err := pgDB.Query(sqlcontent)
		if err != nil {
			log.Println(err)
		}
		for row.Next() {
			var name string
			var pwd string
			err = row.Scan(&name, &pwd)
			mm[name] = pwd
		}
		
		
- sql插入一行
    sqlcontent := `insert into public.stu (name,password) values ($1,$2)`
	stmt, err := pgDB.Prepare(sqlcontent)
	_, err = stmt.Exec(form.User, form.Password)

- sql更新一行
    sqlupdate := `update public.stu set name=$1 where id=$2`
    stmt, err := pgDB.Prepare(sqlupdate)
			_, err = stmt.Exec(form.User, 1)
- sql删除
    sqlDel := `delete from student where id=$1`
    stmt, err := pgDB.Prepare(sqlDel)
	_, err = stmt.Exec(1)
