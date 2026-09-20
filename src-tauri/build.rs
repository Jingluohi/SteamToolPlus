// Tauri + Steam 消息 protobuf 构建脚本
// 1. 调用 tauri_build 生成 Tauri 上下文
// 2. 用 prost-build 将 Steam 官方 .proto 编译为 Rust 结构体（输出到 OUT_DIR）
//    生成的代码由 engine/messages.rs 通过 include! 引入

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri_build::build();

    // Steam 协议 .proto 源文件目录（随项目分发，构建时无需联网）
    let proto_dir = std::path::PathBuf::from("protos");

    // 本引擎仅使用登录 + 客户端服务相关的消息，
    // 其余（商店/好友/云等）不在裁剪范围内，不参与编译
    let proto_files = [
        "steammessages_base.proto",              // CMsgProtoBufHeader / CMsgMulti 等基础消息
        "steammessages_clientserver_login.proto", // CMsgClientLogon / CMsgClientLogonResponse
        "steammessages_clientserver.proto",       // CMsgClientCMList / CMsgClientRequestAppInfo 等
        "encrypted_app_ticket.proto",             // 客户端加密票据（appinfo 依赖）
        "content_manifest.proto",                 // depot manifest V3（payload/metadata/signature）
        "steammessages_content_server_directory.proto", // ContentServerDirectory 服务（CDN 服务器列表 / CDN auth token）
    ];

    let protos: Vec<std::path::PathBuf> = proto_files
        .iter()
        .map(|f| proto_dir.join(f))
        .collect();

    // 若 proto 源文件有更新则触发重新编译
    for proto in &protos {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    let mut config = prost_build::Config::new();
    // 使用随 crate 分发的 protoc（无需系统安装）
    let protoc_path = protoc_bin_vendored::protoc_bin_path()?;
    config.protoc_executable(protoc_path);
    // 兼容旧 protoc 默认行为，减少非法默认值报错
    config.protoc_arg("--experimental_allow_proto3_optional");
    // include 路径指向 protos 根目录，使 "google/protobuf/descriptor.proto" 可解析
    config.compile_protos(&protos, &[proto_dir])?;

    Ok(())
}
