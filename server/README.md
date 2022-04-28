# Hello RSS Server

## 开发指南

### 初始化环境配置文件

```shell
cp .sample.env .env
```

### 数据库开发步骤

- [在entity/src目录新增table_name.rs的Entity文件][1]
- [编写数据库升级和降级程序][2]
- [运行数据库升级和降级测试][3]

[1]: https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli
[2]: https://www.sea-ql.org/SeaORM/docs/migration/writing-migration
[3]: https://www.sea-ql.org/SeaORM/docs/migration/running-migration
