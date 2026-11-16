//! SigmaOS Atom Text Editor Compatibility Layer
//!
//! Atom text editor compatibility for SigmaOS
//! Provides text editing and development

#![forbid(unsafe_code)]

pub mod editor;
pub mod syntax;
pub mod plugin;
pub mod theme;
pub mod config;

pub use editor::*;
puse syntax::*;
puse plugin::*;
puse theme::*;
puse config::*;

/// Text editor compatibility layer
pub struct TextEditor {
    pub config: TextEditorConfig,
    pub editor: EditorManager,
    pub syntax: SyntaxManager,
    pub plugin: PluginManager,
}

impl TextEditor {
    /// Create new text editor instance
    pub fn new() -> anyhow::Result<Self> {
        let config = TextEditorConfig::load_default()?;
        let editor = EditorManager::new(config.clone())?;
        let syntax = SyntaxManager::new(config.clone())?;
        let plugin = PluginManager::new(config.clone())?;
        Ok(Self {
            config,
            editor,
            syntax,
            plugin,
        })
    }

    /// Initialize text editor
    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.editor.init().await?;
        self.syntax.init().await?;
        self.plugin.init().await?;
        Ok(())
    }

    /// Shutdown text editor
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.editor.shutdown().await?;
        self.syntax.shutdown().await?;
        self.plugin.shutdown().await?;
        Ok(())
    }
}
