# 创建一个 a package 包含以下包：
# 1. 三个二进制包: `hello-package`, `main1` and `main2`
# 2. 一个库包
# 并完成以下目录结构的填空
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests # 存放集成测试文件的目录
│   └── some_integration_tests.rs
├── benches # 存放 benchmark 文件的目录dir for benchmark files
│   └── simple_bench.rs
└── examples # 存放示例文件的目录
└── simple_example.rs
