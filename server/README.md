# Hello RSS Server

## 开发指南

### 初始化环境配置文件

```shell
cp .sample.env .env
```

### 数据库开发步骤

* [已有数据库生成Entity或手动编辑Entity][1]
* [编写数据库升级和降级程序][2]
* [运行数据库升级和降级测试][3]

[1]:https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli
[2]:https://www.sea-ql.org/SeaORM/docs/migration/writing-migration
[3]:https://www.sea-ql.org/SeaORM/docs/migration/running-migration


## TODO

### 建立数据库

* [ ] 用户
* [ ] 客户端
* [ ] Token
* [ ] RSS channel
* [ ] RSS item
* [ ] Rss History

### 生成entity

### 生成migration

### 测试migration

### 编写业务逻辑

### 编写接口