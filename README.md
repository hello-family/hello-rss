HelloRSS Server
================================================================


## 开发流程

### 1.创建.env文件

```shell
copy .sample.env .env
```

### 2.运行数据库迁移

```
sea-orm-cli migrate
```

### 3.运行服务