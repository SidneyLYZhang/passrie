# PASSRIE

## PRE

因为很多原因，没有一个满意的密码管理器，试着用过gopass、passpie，但是完全达不到我需要的程度。

gopass很好用，但迁移很难。passpie很现代，但代码bug太多，没有很好的适配性。所以只好自己来重新构建一个CLI版本的密码管理器。

现阶段我只需要比较简单的几个功能：

1. 生成随机密码
2. 分类记录密码
3. 查找密码
4. 更新密码
5. 删除密码
6. 同步密码

最重要的是整体加密处理，可以指定密码库的保存位置，不依赖其他外部产品，纯rust代码，导出可以用于windows的可执行文件。

## 笔记

[基础知识](https://course.rs/about-book.html)

- 加密解密，主要依赖 [rsa库](https://docs.rs/rsa/0.8.2/rsa/) ;
- 命令行处理 [clap](https://docs.rs/clap/latest/clap/) ，参考 [clap入门](https://zhuanlan.zhihu.com/p/57966589) ；
- 数据表管理 [polars](https://pola-rs.github.io/polars/polars/#) ;
- 密码管理 [password_hash](https://github.com/RustCrypto/password-hashes)
- 密码生成 [passwords](https://docs.rs/crate/passwords/3.1.13)
- 数据保存与配置 https://docs.rs/serde_yaml/0.9.21/serde_yaml/ https://docs.rs/rusqlite/latest/rusqlite/

## 开发计划

- [ ] init: 初始化密码库
- [ ] create/new/add: 新增密码
- [ ] generate/gen: 生成密码
- [ ] show/see/copy/cp: 查看、拷贝指定密码
- [ ] encrypt/enc/jia: 加密文件
- [ ] dencrypt/den/jie: 解密文件
- [ ] set: 设定密码管理器
- [ ] search: 搜索一个密码
- [ ] update: 更新密码
- [ ] list/ls:
- [ ] delete/rm:
- [ ] import/put:
- [ ] sync: