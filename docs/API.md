# 前后端交互 API 手册

前后端共有两种方式，分别为前端调用后端的 `invoke` 方法和后端通知前端的 `event`
参考:

- [invoke](https://tauri.app/v1/api/js/tauri#invoke)
- [event](https://tauri.app/v1/api/js/event)

## invoke

### create_tab

```rust
create_tab(file_path: String, content: String) -> bool; 
```

创建一个新文件 tab，file_path 为文件路径，content 为文件内容。

### close_tab

```rust
close_tab(filepath: String) -> Optional;
```

close_tab 方法用于关闭指定的 tab。

成功时，message 返回新聚焦 tab 的 filepath；失败时，message 返回错误信息。

### update_tab

```rust
update_tab(filepath: String, content: String) -> Optional;
```

更新 tab 内容。成功时，该 tab 的dirty bit 会被设置。

### write_tab

```rust
write_tab(filepath: String) -> Optional;
```

保存文件到原本路径。保存失败时，String 返回错误信息。该操作会更新 last_modified 并清除 dirty bit.

### read_tab

```rust
read_tab(filepath: String) -> Optional;
```

从本地重新读取该 tab 的内容。成功时，返回该 tab 的内容；失败时，返回错误信息。

### change_current_tab

```rust
change_current_tab(newpath: String) -> bool;
```

设置新聚焦 tab 的 filepath
当仅后端不存在该 tab 时返回 false，否则返回 true。

## event

### front_file_open

```rust
event: “front_file_open”,
payload: {
  filepath: String,
  content: String,
}
```

### front_file_save

```rust
event: “front_file_save”,
payload: String,
```

## Struct definition

### Optional

```rust
Struct Optional{
  success: bool,
  message: String,
}
```

