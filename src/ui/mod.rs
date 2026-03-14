pub mod assets;
pub mod colors;
pub mod components;
pub mod rootview;
pub mod types;
pub mod views;

// 重新导出 i18n 模块以便 UI 组件使用
pub use crate::i18n::TranslationKey;
