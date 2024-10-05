# seal-story-painter-backend

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

[海豹核心](https://github.com/sealdice/sealdice-core) 的开源跑团日志服务，可自部署启动后端服务和相应的染色器页面。

## 使用

在海豹核心的「高级设置」中配置「自定义后端 URL」为该跑团日志服务的地址，如 `http://localhost:3212/dice/api/log` 。

访问 `http://localhost:3212` 即可进入对应的染色器页面。

## 配置

~~懒了，以后再补充。~~

## 其他说明

暂时将 [染色器前端](https://github.com/sealdice/story-painter) 对应的构建产物，直接放在了本项目的 `static` 中，后期考虑自动化构建。

对染色器前端代码有微调，修改了原代码中硬编码的后端地址。
