# Rust 训练营录播课程表

## 第一周：从 Hello world 到实用的 CLI 工具

1. 了解 Rust 和其他编程语言相比的独特之处，适用场景等。
2. 掌握基础数据结构：String、Vec、HashMap 等等。
3. 掌握 Rust 编程中的常用库。
4. 掌握构建 Rust 程序的基本思路，学会使用 Struct 和定义简单的 trait，了解泛型在 Rust 中的使用，以及能够读懂别人撰写的泛型代码。
5. Rust 开发过程中可能遇到的问题及解决方法。
6. 构建自己的瑞士军刀：命令行工具、base64 编解码等一系列有用的小工具。
7. 构建 HTTP 静态页面服务器。

## 第二周：从单线程到多线程，从同步到异步

1. 掌握线程的使用方法。
2. 掌握 Arc、Mutex、RwLock 等多线程开发必备的基本数据结构。
3. 学会构建并发哈希表。
4. 学会管理线程，掌握线程间的通信、线程调度策略。
5. 了解 Send / Sync trait，知道如何解决常见的多线程代码编译错误。
6. 掌握 Rust 中异步处理方式。
7. 进一步夯实对 trait 的理解。
8. 构建一个简易版 Redis 服务器。

## 第三周：深入浅出元编程

1. 掌握宏的使用方法和适用场景。
2. 掌握元编程的编写技巧和适用场景。
3. 继续探索 Rust 语法和数据结构的应用技巧，理解 Rust 的设计理念。
4. 熟练使用 darling crate，同时掌握 syn 和 quote 这两个基础 crate。
5. 学会使用 Cargo expand 了解 derive macro 背后发生的事情。
6. 学会使用 macro_rules! 处理简单的宏替换。
7. 了解 AST，以及基本的遍历 AST 的方法。

## 第四周：Rust 生态系统概览

1. 错误处理：anyhow、thiserror。
2. 日志处理：tracing、tracing-subscriber。
3. 宏：derive_builder、derive_more、strum、darling。
4. 数据转换：serde 生态。
5. 异步运行时：tokio 生态。
6. 应用开发：tower 生态。
7. 关系型数据库：sqlx 生态。
8. 如何高效利用 Rust 社区信息给自己提供帮助。

## 第五周：构建高性能互联网应用

1. 掌握 axum 框架，学会使用 trait 和宏优雅处理 web 请求。
2. 构建一个高性能聊天系统，学会使用 axum 中 state、extractor、middleware 等基本功能。
3. 将 API 服务与 web 服务器结合，打造生产环境可用的系统。
4. 掌握 axum、tokio、tower 生态间的联动。
5. 掌握 axum 的错误处理，学会如何把 thiserror 定义的错误通过 axum 暴露给 API 用户和最终用户。

## 第六周：构建强大高效的微服务

1. 掌握 prost、tonic、tower、axum 生态间的结合方式。
2. 构建强大的微服务（CRM 服务系统）：数据建模、服务建模，并增加适当的中间件完善系统功能。
3. 掌握处理 json、yaml 等不同类型的文件格式的方法。
4. 完善微服务需要的其他基础服务：授权请求等。

## 第七周：轻松处理各种数据

1. 了解 Rust 对 SIMD 的支持，Rust 如何帮助工程师处理大规模数据。
2. 掌握 apache arrow 生态的内容和特性。
3. 学会使用 arrow、DataFusion、Polars 以及 lance 等工具处理大规模数据。

## 第八周：灵活嵌入各种语法

1. 掌握 nom，能够处理简单的语法解析。
2. 掌握 pest，能够构建自己的语法并解析它。
3. 简单了解如何嵌入 rhai / rlua / pyo3 / rquickjs / deno。
4. 构建一个类似于 deno deploy 的服务，允许用户运行他们的 typescript/javascript 代码。
5. 用 pyo3 和 axum 构建一个类似 nextjs 的 web framework。

## 第九周：让 Rust 代码成为其他语言编写的系统的基石

1. 在 nodejs 中使用 Rust 模块：构建一个为 nodejs 提供 blake3 哈希的支持。
2. 在 python 中使用 Rust 模块：构建一个 python 版本的嵌入式的 Vector DB。

## 第十周：打造跨端的桌面（和移动端）应用

1. 了解 Rust 对跨端的支持和应用。
2. 使用 tauri 构建桌面端和简单的移动端应用：构建原生菜单、交互界面等。
3. 基于 Rust 生态构建应用后端（axum、tokio、tower 等）。
4. 学习掌握 DuckDB。

## 第十一周：构建更好的 ChatGPT: 项目架构和数据建模

1. 构建 Rust 产品前的思考：架构、数据结构、接口设计。
2. 架构设计：根据业务核心流程设计系统架构。
3. 数据结构：设计核心业务流程需要的数据结构和对应的接口。
4. 数据库建模：使用 sqlx 及 postgres 设计数据库。
5. 数据库迁移：学习掌握 sqlx 进行数据库迁移的基本要领。
6. 连接 OpenAI：打造进阶版 ChatGPT。

## 第十二周：构建更好的 ChatGPT: 服务端核心逻辑以及 API 实现

1. 学习使用 smithy，掌握 smithy IDL，使用 smithy 生成 OpenAPI spec。
2. 学习如何在 axum 中通过 tower 接口嵌入 smithy 的支持。
3. 实现架构中需要的 API 接口。
4. 完善用户注册和登录功能。

## 第十三周：构建更好的 ChatGPT: 前端、部署与监控

1. 完善前端页面。
2. 使用 tracing 进行日志处理。
3. 使用 sentry 进行错误处理。
4. 使用 opentelemetry 进行服务指标监控。

## 第十四周：构建更好的 ChatGPT: 数据平台

1. 使用 protobuf 来建模可监控的用户行为。
2. 学习掌握 ClickHouse。
3. 通过开源工具进行数据的可视化分析。

## 加餐（TBD）：构建一个适用于各种软件服务的授权系统

1. 对比授权系统的基本解决思路。
2. 精读 zanzibar 论文。
3. 在理解 zanzibar 的基础上，对其精简，构建相应的 Rust 版本。
4. 使用 nom 或者 pest 构建 zanzibar 模型的解析（user - object - relation - relation tuple）。
5. 从社区中寻找合适的 graph databse 来存储 zanzibar 授权信息。
6. 构建 API 提供授权服务。
7. 产品化 zanzibar 服务的建议（如何达到需要的可扩展性和服务 SLA - e.g. 99.99% uptime）。
