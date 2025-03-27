# open-seal-server

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
![Server](https://img.shields.io/badge/SealDice-Server-blue)

[海豹核心](https://github.com/sealdice/sealdice-core) 的开源后端服务，支持自部署海豹核心依赖的云端服务。

（预计）支持的功能：

- [x] 分发更新：提供对海豹核心本体、更新器、内置客户端等的下载。
- [x] 跑团日志：接收上传的日志文件，并提供对应的染色器页面。
- [x] 公骰：支持注册和上报公骰信息。
- [ ] ~~扩展商店：自部署商店源，支持扩展内容（插件、牌堆）下载。~~
- [ ] 云黑：支持上报和获取黑名单记录。
- [ ] 管理后台：进行后台的配置和管理。

## 配置

~~懒了，以后再补充。~~

## 使用

### 跑团日志

在海豹核心的「高级设置」中配置「自定义后端 URL」为该跑团日志服务的地址，如 `http://localhost:3210/dice/api/log` 。

访问 `http://localhost:3210` 即可进入对应的染色器页面。

## 其他说明

### 1. 染色器前端代码调整

暂时将 [染色器前端](https://github.com/sealdice/story-painter) 对应的构建产物，直接放在了本项目的 `static` 中，后期考虑自动化构建。

对染色器前端代码有微调，修改了原代码中硬编码的后端地址。

### 2. 开发环境搭建

```bash
sea-orm-cli migrate generate <table_name>
DATABASE_URL=sqlite://data.db sea-orm-cli migrate up
sea-orm-cli generate entity -u sqlite://data.db -o entity/src/entities --date-time-crate chrono --with-serde both
```