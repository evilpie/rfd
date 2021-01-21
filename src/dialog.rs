use crate::FileHandle;

use std::path::Path;
use std::path::PathBuf;

pub(crate) struct Filter {
    pub name: String,
    pub extensions: Vec<String>,
}

/// ## Synchronous File Dialog
/// #### Supported Platforms:
/// - Linux
/// - Windows
/// - Mac
#[derive(Default)]
pub struct FileDialog {
    pub(crate) filters: Vec<Filter>,
    pub(crate) starting_directory: Option<PathBuf>,
}

impl FileDialog {
    /// New file dialog builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Add file extension filter.
    ///
    /// Takes in the name of the filter, and list of extensions
    ///
    /// #### Name of the filter will be displayed on supported platforms
    /// - Windows
    /// - Linux
    ///
    /// On platforms that don't support filter names, all filters will be merged into one filter
    pub fn add_filter(mut self, name: &str, extensions: &[&str]) -> Self {
        self.filters.push(Filter {
            name: name.into(),
            extensions: extensions.iter().map(|e| e.to_string()).collect(),
        });
        self
    }

    /// Set starting directory of the dialog.
    /// #### Supported Platforms:
    /// - Linux
    /// - Windows
    /// - Mac
    pub fn set_directory<P: AsRef<Path>>(mut self, path: &P) -> Self {
        self.starting_directory = Some(path.as_ref().into());
        self
    }
}

use crate::backend::FilePickerDialogImpl;

#[cfg(not(target_arch = "wasm32"))]
impl FileDialog {
    /// Pick one file
    pub fn pick_file(self) -> Option<PathBuf> {
        FilePickerDialogImpl::pick_file(self)
    }

    /// Pick multiple files
    pub fn pick_files(self) -> Option<Vec<PathBuf>> {
        FilePickerDialogImpl::pick_files(self)
    }

    /// Pick one folder
    pub fn pick_folder(self) -> Option<PathBuf> {
        crate::backend::pick_folder(self)
    }

    /// Opens save file dialog
    pub fn save_file(self) -> Option<PathBuf> {
        crate::backend::save_file(self)
    }
}

/// ## Asynchronous File Dialog
/// #### Supported Platforms:
/// - Linux
/// - Windows
/// - Mac
/// - WASM32
#[derive(Default)]
pub struct AsyncFileDialog {
    file_dialog: FileDialog,
}

impl AsyncFileDialog {
    /// New file dialog builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Add file extension filter.
    ///
    /// Takes in the name of the filter, and list of extensions
    ///
    /// #### Name of the filter will be displayed on supported platforms
    /// - Windows
    /// - Linux
    ///
    /// On platforms that don't support filter names, all filters will be merged into one filter
    pub fn add_filter(mut self, name: &str, extensions: &[&str]) -> Self {
        self.file_dialog = self.file_dialog.add_filter(name, extensions);
        self
    }

    /// Set starting directory of the dialog.
    /// #### Supported Platforms:
    /// - Linux
    /// - Windows
    /// - Mac
    pub fn set_directory<P: AsRef<Path>>(mut self, path: &P) -> Self {
        self.file_dialog = self.file_dialog.set_directory(path);
        self
    }
}

use crate::backend::AsyncFilePickerDialogImpl;
use std::future::Future;

impl AsyncFileDialog {
    /// Pick one file
    pub fn pick_file(self) -> impl Future<Output = Option<FileHandle>> {
        AsyncFilePickerDialogImpl::pick_file_async(self.file_dialog)

        // crate::backend::pick_file_async(self.file_dialog)
    }

    /// Pick multiple files
    pub fn pick_files(self) -> impl Future<Output = Option<Vec<FileHandle>>> {
        // crate::backend::pick_files_async(self.file_dialog)
        AsyncFilePickerDialogImpl::pick_files_async(self.file_dialog)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Pick one folder
    ///
    /// Does not exist in `WASM32`
    pub fn pick_folder(self) -> impl Future<Output = Option<FileHandle>> {
        crate::backend::pick_folder_async(self.file_dialog)
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Opens save file dialog
    ///
    /// Does not exist in `WASM32`
    pub fn save_file(self) -> impl Future<Output = Option<FileHandle>> {
        crate::backend::save_file_async(self.file_dialog)
    }
}

use crate::MessageDialogImpl;

/// ## Synchronous Message Dialog
#[derive(Default)]
pub struct MessageDialog {
    pub(crate) text: String,
    pub(crate) level: MessageLevel,
    pub(crate) buttons: MessageButtons,
}

impl MessageDialog {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_text(mut self, text: &str) -> Self {
        self.text = text.into();
        self
    }

    pub fn show(self) {
        MessageDialogImpl::show(self)
    }
}

pub enum MessageLevel {
    Info,
    Warning,
    Error,
}

impl Default for MessageLevel {
    fn default() -> Self {
        Self::Info
    }
}

pub enum MessageButtons {
    Ok,
    OkCancle,
    YesNo,
}

impl Default for MessageButtons {
    fn default() -> Self {
        Self::Ok
    }
}
