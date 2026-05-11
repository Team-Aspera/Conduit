# Changelog

## [0.2.3] - 2025-03

### Added
- Settings page to configure window close behavior (minimize to tray / quit)
- Display current active share IP and interface in Network Share page

### Fixed
- Port forwarder localization and arrow icon rendering issues
- Remove assigned IP address when stopping network share

## [0.2.2] - 2025-02

### Added
- Install and uninstall scripts with desktop entry support
- Logo images integrated into sidebar and About page

### Changed
- Downscaled logo images to improve About page rendering performance
- Made system monitor refresh asynchronous

### Fixed
- UDP forwarder subtask termination logic
- Graceful exit with iptables cleanup

## [0.2.1] - 2025-01

### Added
- Windows support framework with conditional compilation
- Makefile for cross-compilation

### Fixed
- Bundled LXGW WenKai Lite font to fix Chinese character rendering
- Bundled Noto Sans Symbols 2 to fix icon rendering
- iptables bad rule warnings during cleanup

### Changed
- UI overhaul with sidebar navigation and card layouts

## [0.2.0] - 2024-12

### Added
- System Monitor page with auto-refresh
- i18n support (Chinese / English)
- Config import/export using file picker
- Detection status for active NAT rules

### Fixed
- Chinese character rendering (bundled Noto Sans CJK SC font)
- Detect Status reliability and state preservation

## [0.1.0] - 2024-11

### Added
- Initial release
- System network share (NAT) via iptables
- Multi-task TCP and UDP port forwarding
- Modern UI built with Iced framework
- Async engine powered by Tokio
