# axum-web

**axum-web基于axum库开发web服务，实现了登陆认证、redis缓存、接口token认证，文件上传、数据库操作等等功能**






```
axum-web/
├── application.toml
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
└── src
    ├── main.rs
    └── web
        ├── configuration.rs
        ├── database
        │   └── mod.rs
        ├── datasource.rs
        ├── jsons
        │   ├── mod.rs
        │   └── user.rs
        ├── login
        │   ├── login.rs
        │   └── mod.rs
        ├── mod.rs
        ├── paths
        │   ├── mod.rs
        │   └── path.rs
        ├── result.rs
        ├── security
        │   ├── check_token.rs
        │   └── mod.rs
        ├── server.rs
        └── upload
            ├── mod.rs
            └── upload.rs
```




## 献给小鸭鸭