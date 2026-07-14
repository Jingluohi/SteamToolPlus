// 服务模块
// 实现所有业务逻辑，通过trait定义接口抽象

pub mod cache_service;
pub mod config_service;
pub mod download_service;
pub mod fake_imported_service;
pub mod game_data_service;
pub mod game_service;
pub mod opensteamtool_service;
pub mod window_service;

pub use cache_service::*;
pub use config_service::*;
pub use download_service::*;
pub use game_service::*;
pub use window_service::*;
