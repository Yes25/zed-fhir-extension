use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, LanguageServerId,
    LanguageServerInstallationStatus, Os, Result, serde_json, settings::LspSettings,
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

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("fhir-lsp", worktree)?;
        Ok(settings.initialization_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("fhir-lsp", worktree)?;
        Ok(settings.settings)
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

        let (os, arch) = zed::current_platform();

        let asset_name = match (&os, &arch) {
            (Os::Mac, Architecture::Aarch64) => "fhir-lsp-aarch64-apple-darwin.tar.gz",
            (Os::Mac, _) => "fhir-lsp-x86_64-apple-darwin.tar.gz",
            (Os::Linux, Architecture::Aarch64) => "fhir-lsp-aarch64-unknown-linux-gnu.tar.gz",
            (Os::Linux, _) => "fhir-lsp-x86_64-unknown-linux-gnu.tar.gz",
            (Os::Windows, _) => "fhir-lsp-x86_64-pc-windows-msvc.zip",
        };

        // Bump this version when a new release is published to trigger a re-download for users.
        let version = "0.1.0";
        let url = format!(
            "https://github.com/Yes25/fhir-lsp/releases/download/v{version}/{asset_name}"
        );
        let file_type = match os {
            Os::Windows => DownloadedFileType::Zip,
            _ => DownloadedFileType::GzipTar,
        };

        let download_dir = format!("bin/fhir-lsp-{version}");
        let binary_name = match os {
            Os::Windows => "fhir-lsp.exe",
            _ => "fhir-lsp",
        };
        let binary_path = format!("{download_dir}/{binary_name}");

        if !std::fs::metadata(&binary_path).is_ok_and(|m| m.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(&url, &download_dir, file_type).map_err(|e| {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &LanguageServerInstallationStatus::Failed(e.clone()),
                );
                format!("failed to download fhir-lsp: {e}")
            })?;

            zed::make_file_executable(&binary_path)?;

            zed::set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::None,
            );
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

zed::register_extension!(Fhir);
