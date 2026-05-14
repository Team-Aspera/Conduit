use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;
use tokio::sync::watch;
use uuid::Uuid;

use crate::network::SystemReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Chinese,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloseBehavior {
    Minimize,
    Quit,
}

impl Language {
    pub fn get(&self, key: &str) -> &'static str {
        crate::i18n::get(*self, key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    SystemForward,
    PortForward,
    SystemMonitor,
    About,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Tcp => write!(f, "TCP"),
            Protocol::Udp => write!(f, "UDP"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PortForwarderConfig {
    pub protocol: Protocol,
    pub src_addr: String,
    pub src_port: String,
    pub dst_addr: String,
    pub dst_port: String,
}

pub struct PortForwarder {
    pub id: Uuid,
    pub protocol: Protocol,
    pub src_addr: String,
    pub src_port: String,
    pub dst_addr: String,
    pub dst_port: String,
    pub is_active: bool,
    pub status: Cow<'static, str>,
    pub stop_tx: Option<watch::Sender<bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanShareConfig {
    pub interface: String,
    pub ip: String,
    pub mask: String,
    pub wans: Vec<String>,
}

pub struct LanShare {
    pub config: LanShareConfig,
    pub is_active: bool,
    pub status: std::borrow::Cow<'static, str>,
    #[allow(dead_code)]
    pub stop_tx: Option<tokio::sync::watch::Sender<bool>>,
}

impl Default for LanShareConfig {
    fn default() -> Self {
        Self {
            interface: String::new(),
            ip: "192.168.10.1".into(),
            mask: "24".into(),
            wans: vec![],
        }
    }
}

fn config_path() -> PathBuf {
    if cfg!(test) {
        return std::env::temp_dir().join("conduit-test-config.json");
    }
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    let path = base.join("conduit").join("config.json");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    path
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub language: Language,
    pub close_behavior: CloseBehavior,
    pub forwarders: Vec<PortForwarderConfig>,
    pub lan_shares: Vec<LanShareConfig>,
}

impl AppConfig {
    pub fn load() -> Self {
        if cfg!(test) {
            return Self::default();
        }
        fs::read_to_string(config_path())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(config_path(), json);
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            language: Language::Chinese,
            close_behavior: CloseBehavior::Quit,
            forwarders: vec![],
            lan_shares: vec![LanShareConfig::default()],
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SwitchPage(Page),
    SetCloseBehavior(CloseBehavior),
    CloseRequested,
    TrayClicked,
    #[allow(dead_code)]
    WanToggled(String, bool),
    AddLanShare,
    RemoveLanShare(usize),
    UpdateLanShare(usize, String, String),
    LanWanToggled(usize, String, bool),
    ToggleLanShare(usize),
    #[allow(dead_code)]
    ToggleSysForwarding,
    SysForwardingResult(usize, bool, Result<(), String>),
    DetectSystemForward,
    RefreshInterfaces,
    RefreshSystemReport,
    SystemReportReceived(SystemReport),
    SetRefreshInterval(u64),
    AddForwarder,
    RemoveForwarder(Uuid),
    SrcAddrChanged(Uuid, String),
    SrcPortChanged(Uuid, String),
    DstAddrChanged(Uuid, String),
    DstPortChanged(Uuid, String),
    TogglePortForwarding(Uuid),
    PortForwardingResult(Uuid, Result<(), String>),
    ImportConfig,
    ConfigFileSelected(Option<PathBuf>),
    ExportConfig,
    ConfigFileToExportSelected(Option<PathBuf>),
    LanguageChanged(Language),
    Exit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_all_keys_non_empty() {
        let keys = [
            "nav_share",
            "nav_forward",
            "nav_monitor",
            "nav_about",
            "nav_settings",
            "title_share",
            "title_forward",
            "title_monitor",
            "title_settings",
            "label_wan",
            "label_lan",
            "label_lan_ip",
            "btn_start_share",
            "btn_stop_share",
            "btn_detect",
            "btn_refresh_iface",
            "btn_refresh",
            "btn_add_new",
            "btn_import",
            "btn_export",
            "status_ready",
            "status_active",
            "label_ip_forward",
            "label_enabled",
            "label_disabled",
            "monitor_active_flows",
            "monitor_nat_rules",
            "monitor_port_rules",
            "monitor_listen_ports",
            "msg_det_failed",
            "msg_select_wan",
            "msg_select_lan",
            "msg_stopping",
            "msg_starting",
            "msg_stopped",
            "msg_active_bang",
            "about_desc",
            "label_current_share",
            "label_active_iface",
            "status_running",
            "status_invalid_port",
            "status_stopped",
            "status_imported",
            "label_close_behavior",
            "opt_minimize",
            "opt_quit",
        ];
        for lang in [Language::Chinese, Language::English] {
            for key in &keys {
                let val = lang.get(key);
                assert!(!val.is_empty(), "{:?}.get({:?}) empty", lang, key);
                assert_ne!(val, "Unknown", "{:?}.get({:?}) is Unknown", lang, key);
            }
        }
    }

    #[test]
    fn test_language_unknown_key() {
        assert_eq!(Language::Chinese.get("nonexistent"), "Unknown");
        assert_eq!(Language::English.get("nonexistent"), "Unknown");
    }

    #[test]
    fn test_protocol_display() {
        assert_eq!(Protocol::Tcp.to_string(), "TCP");
        assert_eq!(Protocol::Udp.to_string(), "UDP");
    }

    #[test]
    fn test_close_behavior_serde() {
        let cases = [
            (CloseBehavior::Minimize, "\"Minimize\""),
            (CloseBehavior::Quit, "\"Quit\""),
        ];
        for (val, expected_json) in &cases {
            let json = serde_json::to_string(val).unwrap();
            assert_eq!(json, *expected_json);
            let back: CloseBehavior = serde_json::from_str(&json).unwrap();
            assert_eq!(back, *val);
        }
    }

    #[test]
    fn test_port_forwarder_config_serde() {
        let config = PortForwarderConfig {
            protocol: Protocol::Tcp,
            src_addr: "0.0.0.0".into(),
            src_port: "8080".into(),
            dst_addr: "192.168.1.100".into(),
            dst_port: "80".into(),
        };
        let json = serde_json::to_string_pretty(&config).unwrap();
        let back: PortForwarderConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.protocol, Protocol::Tcp);
        assert_eq!(back.src_addr, "0.0.0.0");
        assert_eq!(back.src_port, "8080");
        assert_eq!(back.dst_addr, "192.168.1.100");
        assert_eq!(back.dst_port, "80");
    }

    #[test]
    fn test_port_forwarder_config_list_serde() {
        let configs = vec![
            PortForwarderConfig {
                protocol: Protocol::Tcp,
                src_addr: "0.0.0.0".into(),
                src_port: "8080".into(),
                dst_addr: "10.0.0.1".into(),
                dst_port: "80".into(),
            },
            PortForwarderConfig {
                protocol: Protocol::Udp,
                src_addr: "0.0.0.0".into(),
                src_port: "5353".into(),
                dst_addr: "10.0.0.1".into(),
                dst_port: "53".into(),
            },
        ];
        let json = serde_json::to_string_pretty(&configs).unwrap();
        let back: Vec<PortForwarderConfig> = serde_json::from_str(&json).unwrap();
        assert_eq!(back.len(), 2);
        assert_eq!(back[0].protocol, Protocol::Tcp);
        assert_eq!(back[1].protocol, Protocol::Udp);
    }
}
