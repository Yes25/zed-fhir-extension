use std::collections::binary_heap;

use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, LanguageServerId,
    LanguageServerInstallationStatus, Os, Result,
};

struct Fhir {
    cached_binary_path: Option<String>,
}

impl zed::Extension for Fhir {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary(language_server_id)?,
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

impl Fhir {
    fn language_server_binary(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).is_ok_and(|m| m.is_file()) {
                return Ok(path.clone());
            }
        }

        // let (os, arch) = zed::current_platform();

        // let binary_name = match (&os, &arch) {
        //     (Os::Mac, Architecture::Aarch64) => "hl7_v2_lsp-aarch64-apple-darwin.tar.gz",
        //     (Os::Mac, _) => "hl7_v2_lsp-x86_64-apple-darwin.tar.gz",
        //     (Os::Linux, Architecture::Aarch64) => "hl7_v2_lsp-aarch64-unknown-linux-gnu.tar.gz",
        //     (Os::Linux, _) => "hl7_v2_lsp-x86_64-unknown-linux-gnu.tar.gz",
        //     (Os::Windows, _) => "hl7_v2_lsp-x86_64-pc-windows-msvc.zip",
        // };

        // // If there is a new LSP release, bumb the version here -> new download of current LSP should be triggered
        // // for users then.
        // let version = "0.1.0";
        // let url = format!(
        //     "https://github.com/Yes25/hl7_v2_lsp/releases/download/v{version}/{binary_name}"
        // );
        // let file_type = match os {
        //     Os::Windows => DownloadedFileType::Zip,
        //     _ => DownloadedFileType::GzipTar,
        // };

        // let download_dir = format!("bin/hl7_v2_lsp-{version}");
        // let binary_path = format!("{download_dir}/hl7_v2_lsp");

        // if !std::fs::metadata(&binary_path).is_ok_and(|m| m.is_file()) {
        //     zed::set_language_server_installation_status(
        //         language_server_id,
        //         &LanguageServerInstallationStatus::Downloading,
        //     );

        //     zed::download_file(&url, &download_dir, file_type).map_err(|e| {
        //         zed::set_language_server_installation_status(
        //             language_server_id,
        //             &LanguageServerInstallationStatus::Failed(e.clone()),
        //         );
        //         format!("failed to download hl7_v2_lsp: {e}")
        //     })?;

        //     zed::make_file_executable(&binary_path)?;

        //     zed::set_language_server_installation_status(
        //         language_server_id,
        //         &LanguageServerInstallationStatus::None,
        //     );
        // }

        let binary_path =
            "/Users/jesse/Code/tree_sitter/fhir-lsp/target/release/fhir-lsp".to_owned();

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

zed::register_extension!(Fhir);
