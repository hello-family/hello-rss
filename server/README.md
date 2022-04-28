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

### 数据库设计

* [ ] user
* [ ] client
* [ ] user_session
* [ ] rss_channel
* [ ] rss_item
* [ ] favorite

### 用户接口

* [ ] user/signup 注册
* [ ] user/activate 激活
* [ ] user/login 登录
* [ ] user/info 获取用户信息
* [ ] user/logout 登出
* [ ] user/change_password 修改密码
* [ ] user/change_email 修改邮箱
* [ ] user/change_email_confirm 修改邮箱确认
* [ ] user/reset_password 重置密码
* [ ] user/reset_password_confirm 重置密码确认

### RSS接口

* [ ] rss/channel/list 获取用户的RSS频道列表
* [ ] rss/channel/add 添加RSS频道
* [ ] rss/channel/delete 删除RSS频道
* [ ] rss/channel/update 更新RSS频道
* [ ] rss/channel/info 获取RSS频道信息
* [ ] rss/channel/fetch 更新RSS频道内容
* [ ] rss/channel/read_all 标记全部已读
* [ ] rss/item/list 获取RSS频道的RSS项目列表
* [ ] rss/item/read 标记条目已读
* [ ] rss/item/favorite 收藏
