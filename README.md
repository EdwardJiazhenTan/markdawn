# 🚀 实时Markdown渲染器

这是一个用Rust构建的实时markdown渲染器，类似React开发环境的热重载功能。

## ✨ 功能特性

- **实时预览**: 保存markdown文件后浏览器立即更新
- **WebSocket连接**: 服务器和客户端之间的实时通信  
- **文件监控**: 自动检测markdown文件的变化
- **多客户端支持**: 多个浏览器窗口同步更新

## 🛠️ 技术栈

- **后端**: Rust + Axum + WebSocket + notify
- **前端**: HTML + JavaScript + WebSocket API
- **解析器**: 自定义markdown解析器

## 🚀 运行方法

```bash
# 启动服务器
cargo run

# 打开浏览器访问
http://localhost:5000

# 编辑这个文件并保存，看看浏览器是否实时更新！
```

## 📝 测试步骤

1. 启动服务器: `cargo run`
2. 浏览器打开: `http://localhost:5000`
3. 编辑 `test.md` 或 `README.md` 文件
4. 保存文件
5. 观察浏览器实时更新！

## 🎯 当前状态

✅ WebSocket服务器  
✅ 文件监控系统  
✅ Markdown解析器  
✅ 实时渲染  
✅ 前端WebSocket客户端

## 🧪 测试这个功能

**试试编辑这段文字，然后保存文件，看看浏览器会不会立即更新！**

时间戳: `2025-08-03` 

如果你看到这个时间戳在浏览器中更新了，说明实时渲染工作正常！🎉