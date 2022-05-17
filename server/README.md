# Hello RSS Server

## 开发指南

### 初始化环境配置文件

```shell
cp .sample.env .env
```

### 数据库开发步骤

1. [在 entity/src 新增 Entity][1]
2. [在 migration/src 新增 migration][2]
3. [进入 migration 目录运行`cargo run`进行数据库迁移][3]

[1]: https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli
[2]: https://www.sea-ql.org/SeaORM/docs/migration/writing-migration
[3]: https://www.sea-ql.org/SeaORM/docs/migration/running-migration

## TODO

### 数据库设计

- [x] user
- [x] client
- [x] access_token
- [x] rss_channel
- [x] rss_item
- [x] favorite

### user 接口

- [ ] user/signup 注册
- [ ] user/activate 激活
- [ ] user/login 登录
- [ ] user/info 获取用户信息
- [ ] user/logout 登出
- [ ] user/change_password 修改密码
- [ ] user/change_email 修改邮箱
- [ ] user/change_email_confirm 修改邮箱确认
- [ ] user/reset_password 重置密码
- [ ] user/reset_password_confirm 重置密码确认

### rss 接口

- [ ] rss/channel/add 添加 RSS 频道
- [ ] rss/channel/delete 删除 RSS 频道
- [ ] rss/channel/update 更新 RSS 频道
- [ ] rss/channel/list 获取用户的 RSS 频道列表
- [ ] rss/channel/info 获取 RSS 频道信息
- [ ] rss/channel/fetch 更新 RSS 频道内容
- [ ] rss/channel/read_all 标记全部已读
- [ ] rss/item/list 获取 RSS 频道的 RSS 项目列表
- [ ] rss/item/read 标记条目已读

### favorite 接口

- [ ] favorite/add 新增收藏
- [ ] favorite/list 获取收藏列表
- [ ] favorite/delete 取消收藏

## 开发参考

- [Axum JWT example](https://github.com/z4rx/axum_jwt_example)
- [Book management](https://github.com/lz1998/axum-book-management)
- [refinery](https://github.com/rust-db/refinery)
- [barrel](https://crates.io/crates/barrel)