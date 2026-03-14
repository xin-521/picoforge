use std::collections::HashMap;
use std::sync::OnceLock;

/// 支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    /// 从语言代码创建 Language
    pub fn from_code(code: &str) -> Self {
        match code {
            "zh" | "zh-CN" | "zh_CN" => Language::Chinese,
            _ => Language::English,
        }
    }

    /// 获取语言代码
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh",
        }
    }
}

/// 翻译键
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranslationKey {
    // 通用
    AppName,
    Refresh,
    Expand,
    Collapse,
    NoDeviceConnected,
    NotAvailable,
    
    // 侧边栏菜单
    MenuHome,
    MenuPasskeys,
    MenuConfiguration,
    MenuSecurity,
    MenuAbout,
    MenuLogs,
    
    // 设备状态
    DeviceStatus,
    StatusOnline,
    StatusOnlineFido,
    StatusError,
    StatusOffline,
    
    // Home 视图 - 页面标题
    HomeTitle,
    HomeDescription,
    
    // Home 视图 - 卡片标题
    DeviceInformation,
    Fido2Information,
    LedConfiguration,
    SecurityStatus,
    
    // Home 视图 - 设备信息
    SerialNumber,
    FirmwareVersion,
    VidPid,
    ProductName,
    FlashMemory,
    
    // Home 视图 - FIDO 信息
    Aaguid,
    FidoVersions,
    PinSet,
    ResidentKeys,
    MinPinLength,
    RemainingCredentials,
    PinSetLabel,
    NotSet,
    Supported,
    NotSupported,
    
    // Home 视图 - LED 配置
    LedGpioPin,
    LedBrightness,
    PresenceTouchTimeout,
    LedDimmable,
    LedSteadyMode,
    FidoModeNotice,
    
    // Home 视图 - 安全状态
    BootMode,
    SecureBoot,
    Development,
    DebugInterface,
    ReadoutLocked,
    DebugEnabled,
    SecureLock,
    Acknowledged,
    Pending,
    
    // 按钮和操作
    Yes,
    No,
    On,
    Off,
    Set,
    Save,
    Cancel,
    Delete,
    Edit,
    Add,
    
    // 通知和消息
    NotificationSuccess,
    NotificationError,
    NotificationWarning,
    NotificationInfo,
    
    // About 视图
    AboutTitle,
    AboutDescription,
    AboutTagline,
    CodeBy,
    Copyright,
    
    // Configuration 视图
    ConfigTitle,
    ConfigDescription,
    VendorPreset,
    VendorIdHex,
    ProductIdHex,
    LedSettings,
    LedGpio,
    LedDriver,
    LedBrightnessLabel,
    TouchTimeoutLabel,
    LedDimmableLabel,
    LedSteadyLabel,
    PowerCycleLabel,
    EnableSecp256k1Label,
    ApplyChanges,
    ApplyingConfig,
    ConfigAppliedSuccess,
    ConfigApplyFailed,
    AuthRequired,
    EnterPinToApply,
    Confirm,
    Unlock,
    
    // Passkeys 视图
    PasskeysTitle,
    PasskeysDescription,
    UnlockStorage,
    LockStorage,
    EnterPinToView,
    StorageUnlocked,
    StorageLocked,
    NoCredentials,
    CredentialRpId,
    CredentialUser,
    CredentialCreated,
    DeletePasskey,
    DeleteConfirm,
    DeleteConfirmMsg,
    ChangePin,
    SetupPin,
    CurrentPin,
    NewPin,
    ConfirmNewPin,
    PinChangedSuccess,
    PinSetupSuccess,
    PinDoNotMatch,
    PinMinLength,
    UpdateMinPinLength,
    MinPinLengthUpdated,
    
    // Security 视图
    SecurityTitle,
    SecurityDescription,
    FeatureUnstable,
    FeatureDisabledWarning,
    LockSettings,
    EnableSecureBootLabel,
    EnableSecureBootDesc,
    SecureLockLabel,
    SecureLockDesc,
    UnderstandRisks,
    PermanentlyLockDevice,
    
    // Passkeys 视图 - 额外键
    ActionDelete,
    ActionCancel,
    ActionUpdate,
    SetMinPinLengthDesc,
    NewPinMinChars,
    PinIsSet,
    NoPinConfigured,
    CurrentMinLength,
    UnlockToViewPasskeys,
    StoredPasskeys,
    ManageCredentialsDesc,
    Unlocked,
    CredentialsStored,
    
    // 通用对话框
    Loading,
    Success,
    Error,
    Warning,
}

impl TranslationKey {
    /// 获取翻译键的字符串标识符
    pub fn as_str(&self) -> &'static str {
        match self {
            TranslationKey::AppName => "app.name",
            TranslationKey::Refresh => "action.refresh",
            TranslationKey::Expand => "action.expand",
            TranslationKey::Collapse => "action.collapse",
            TranslationKey::NoDeviceConnected => "device.no_device",
            TranslationKey::NotAvailable => "common.not_available",
            
            TranslationKey::MenuHome => "menu.home",
            TranslationKey::MenuPasskeys => "menu.passkeys",
            TranslationKey::MenuConfiguration => "menu.configuration",
            TranslationKey::MenuSecurity => "menu.security",
            TranslationKey::MenuAbout => "menu.about",
            TranslationKey::MenuLogs => "menu.logs",
            
            TranslationKey::DeviceStatus => "device.status",
            TranslationKey::StatusOnline => "device.status.online",
            TranslationKey::StatusOnlineFido => "device.status.online_fido",
            TranslationKey::StatusError => "device.status.error",
            TranslationKey::StatusOffline => "device.status.offline",
            
            TranslationKey::HomeTitle => "home.title",
            TranslationKey::HomeDescription => "home.description",
            
            TranslationKey::DeviceInformation => "card.device_info",
            TranslationKey::Fido2Information => "card.fido2_info",
            TranslationKey::LedConfiguration => "card.led_config",
            TranslationKey::SecurityStatus => "card.security_status",
            
            TranslationKey::SerialNumber => "device.serial",
            TranslationKey::FirmwareVersion => "device.firmware",
            TranslationKey::VidPid => "device.vid_pid",
            TranslationKey::ProductName => "device.product_name",
            TranslationKey::FlashMemory => "device.flash_memory",
            
            TranslationKey::Aaguid => "fido.aaguid",
            TranslationKey::FidoVersions => "fido.versions",
            TranslationKey::PinSet => "fido.pin_set",
            TranslationKey::ResidentKeys => "fido.resident_keys",
            TranslationKey::MinPinLength => "fido.min_pin_length",
            TranslationKey::RemainingCredentials => "fido.remaining_credentials",
            TranslationKey::PinSetLabel => "fido.pin_set_label",
            TranslationKey::NotSet => "status.not_set",
            TranslationKey::Supported => "status.supported",
            TranslationKey::NotSupported => "status.not_supported",
            
            TranslationKey::LedGpioPin => "led.gpio_pin",
            TranslationKey::LedBrightness => "led.brightness",
            TranslationKey::PresenceTouchTimeout => "led.touch_timeout",
            TranslationKey::LedDimmable => "led.dimmable",
            TranslationKey::LedSteadyMode => "led.steady_mode",
            TranslationKey::FidoModeNotice => "led.fido_mode_notice",
            
            TranslationKey::BootMode => "security.boot_mode",
            TranslationKey::SecureBoot => "security.secure_boot",
            TranslationKey::Development => "security.development",
            TranslationKey::DebugInterface => "security.debug_interface",
            TranslationKey::ReadoutLocked => "security.readout_locked",
            TranslationKey::DebugEnabled => "security.debug_enabled",
            TranslationKey::SecureLock => "security.secure_lock",
            TranslationKey::Acknowledged => "security.acknowledged",
            TranslationKey::Pending => "security.pending",
            
            TranslationKey::Yes => "common.yes",
            TranslationKey::No => "common.no",
            TranslationKey::On => "common.on",
            TranslationKey::Off => "common.off",
            TranslationKey::Set => "common.set",
            TranslationKey::Save => "action.save",
            TranslationKey::Cancel => "action.cancel",
            TranslationKey::Delete => "action.delete",
            TranslationKey::Edit => "action.edit",
            TranslationKey::Add => "action.add",
            
            TranslationKey::NotificationSuccess => "notification.success",
            TranslationKey::NotificationError => "notification.error",
            TranslationKey::NotificationWarning => "notification.warning",
            TranslationKey::NotificationInfo => "notification.info",
            
            // About 视图
            TranslationKey::AboutTitle => "about.title",
            TranslationKey::AboutDescription => "about.description",
            TranslationKey::AboutTagline => "about.tagline",
            TranslationKey::CodeBy => "about.code_by",
            TranslationKey::Copyright => "about.copyright",
            
            // Configuration 视图
            TranslationKey::ConfigTitle => "config.title",
            TranslationKey::ConfigDescription => "config.description",
            TranslationKey::VendorPreset => "config.vendor_preset",
            TranslationKey::VendorIdHex => "config.vendor_id",
            TranslationKey::ProductIdHex => "config.product_id",
            TranslationKey::LedSettings => "config.led_settings",
            TranslationKey::LedGpio => "config.led_gpio",
            TranslationKey::LedDriver => "config.led_driver",
            TranslationKey::LedBrightnessLabel => "config.led_brightness",
            TranslationKey::TouchTimeoutLabel => "config.touch_timeout",
            TranslationKey::LedDimmableLabel => "config.led_dimmable",
            TranslationKey::LedSteadyLabel => "config.led_steady",
            TranslationKey::PowerCycleLabel => "config.power_cycle",
            TranslationKey::EnableSecp256k1Label => "config.enable_secp256k1",
            TranslationKey::ApplyChanges => "config.apply_changes",
            TranslationKey::ApplyingConfig => "config.applying",
            TranslationKey::ConfigAppliedSuccess => "config.success",
            TranslationKey::ConfigApplyFailed => "config.failed",
            TranslationKey::AuthRequired => "config.auth_required",
            TranslationKey::EnterPinToApply => "config.enter_pin",
            TranslationKey::Confirm => "action.confirm",
            TranslationKey::Unlock => "action.unlock",
            
            // Passkeys 视图
            TranslationKey::PasskeysTitle => "passkeys.title",
            TranslationKey::PasskeysDescription => "passkeys.description",
            TranslationKey::UnlockStorage => "passkeys.unlock_storage",
            TranslationKey::LockStorage => "passkeys.lock_storage",
            TranslationKey::EnterPinToView => "passkeys.enter_pin",
            TranslationKey::StorageUnlocked => "passkeys.storage_unlocked",
            TranslationKey::StorageLocked => "passkeys.storage_locked",
            TranslationKey::NoCredentials => "passkeys.no_credentials",
            TranslationKey::CredentialRpId => "passkeys.rp_id",
            TranslationKey::CredentialUser => "passkeys.user",
            TranslationKey::CredentialCreated => "passkeys.created",
            TranslationKey::DeletePasskey => "passkeys.delete",
            TranslationKey::DeleteConfirm => "passkeys.delete_confirm",
            TranslationKey::DeleteConfirmMsg => "passkeys.delete_confirm_msg",
            TranslationKey::ChangePin => "passkeys.change_pin",
            TranslationKey::SetupPin => "passkeys.setup_pin",
            TranslationKey::CurrentPin => "passkeys.current_pin",
            TranslationKey::NewPin => "passkeys.new_pin",
            TranslationKey::ConfirmNewPin => "passkeys.confirm_pin",
            TranslationKey::PinChangedSuccess => "passkeys.pin_changed",
            TranslationKey::PinSetupSuccess => "passkeys.pin_setup",
            TranslationKey::PinDoNotMatch => "passkeys.pin_mismatch",
            TranslationKey::PinMinLength => "passkeys.pin_min_length",
            TranslationKey::UpdateMinPinLength => "passkeys.update_min_length",
            TranslationKey::MinPinLengthUpdated => "passkeys.min_length_updated",
            
            // Security 视图
            TranslationKey::SecurityTitle => "security_view.title",
            TranslationKey::SecurityDescription => "security_view.description",
            TranslationKey::FeatureUnstable => "security_view.feature_unstable",
            TranslationKey::FeatureDisabledWarning => "security_view.disabled_warning",
            TranslationKey::LockSettings => "security_view.lock_settings",
            TranslationKey::EnableSecureBootLabel => "security_view.enable_secure_boot",
            TranslationKey::EnableSecureBootDesc => "security_view.enable_secure_boot_desc",
            TranslationKey::SecureLockLabel => "security_view.secure_lock",
            TranslationKey::SecureLockDesc => "security_view.secure_lock_desc",
            TranslationKey::UnderstandRisks => "security_view.understand_risks",
            TranslationKey::PermanentlyLockDevice => "security_view.lock_device",
            
            // Passkeys 视图 - 额外键
            TranslationKey::ActionDelete => "action.delete",
            TranslationKey::ActionCancel => "action.cancel",
            TranslationKey::ActionUpdate => "action.update",
            TranslationKey::SetMinPinLengthDesc => "passkeys.set_min_pin_length_desc",
            TranslationKey::NewPinMinChars => "passkeys.new_pin_min_chars",
            TranslationKey::PinIsSet => "passkeys.pin_is_set",
            TranslationKey::NoPinConfigured => "passkeys.no_pin_configured",
            TranslationKey::CurrentMinLength => "passkeys.current_min_length",
            TranslationKey::UnlockToViewPasskeys => "passkeys.unlock_to_view",
            TranslationKey::StoredPasskeys => "passkeys.stored_passkeys",
            TranslationKey::ManageCredentialsDesc => "passkeys.manage_credentials_desc",
            TranslationKey::Unlocked => "passkeys.unlocked",
            TranslationKey::CredentialsStored => "passkeys.credentials_stored",
            
            // 通用对话框
            TranslationKey::Loading => "common.loading",
            TranslationKey::Success => "common.success",
            TranslationKey::Error => "common.error",
            TranslationKey::Warning => "common.warning",
        }
    }
}

/// 翻译器
pub struct Translator {
    language: Language,
    translations: HashMap<String, String>,
}

static TRANSLATOR: OnceLock<Translator> = OnceLock::new();

impl Translator {
    /// 创建新的翻译器
    pub fn new(language: Language) -> Self {
        let translations = load_translations(language);
        Self {
            language,
            translations,
        }
    }

    /// 初始化全局翻译器
    pub fn init(language: Language) {
        let translator = Self::new(language);
        TRANSLATOR.set(translator).ok();
    }

    /// 获取全局翻译器
    pub fn global() -> &'static Self {
        TRANSLATOR.get().expect("Translator not initialized")
    }

    /// 翻译单个键
    pub fn t(&self, key: TranslationKey) -> &str {
        self.translations
            .get(key.as_str())
            .map(|s| s.as_str())
            .unwrap_or(key.as_str())
    }

    /// 翻译带参数的文本
    pub fn t_with_args(&self, key: TranslationKey, args: &[&str]) -> String {
        let template = self.t(key);
        args.iter()
            .enumerate()
            .fold(template.to_string(), |acc, (i, arg)| {
                acc.replace(&format!("{{{}}}", i), arg)
            })
    }

    /// 获取当前语言
    pub fn language(&self) -> Language {
        self.language
    }

    /// 切换语言
    pub fn set_language(&mut self, language: Language) {
        self.language = language;
        self.translations = load_translations(language);
    }
}

/// 加载翻译
fn load_translations(language: Language) -> HashMap<String, String> {
    match language {
        Language::English => load_english_translations(),
        Language::Chinese => load_chinese_translations(),
    }
}

/// 英文翻译
fn load_english_translations() -> HashMap<String, String> {
    [
        ("app.name", "PicoForge"),
        ("action.refresh", "Refresh"),
        ("action.expand", "Expand"),
        ("action.collapse", "Collapse"),
        ("device.no_device", "No Device Connected"),
        ("common.not_available", "Not Available"),
        
        ("menu.home", "Home"),
        ("menu.passkeys", "Passkeys"),
        ("menu.configuration", "Configuration"),
        ("menu.security", "Security"),
        ("menu.about", "About"),
        ("menu.logs", "Logs"),
        
        ("device.status", "Device Status"),
        ("device.status.online", "Online"),
        ("device.status.online_fido", "Online - Fido"),
        ("device.status.error", "Error"),
        ("device.status.offline", "Offline"),
        
        ("home.title", "Device Overview"),
        ("home.description", "Quick view of your device status and specifications."),
        
        ("card.device_info", "Device Information"),
        ("card.fido2_info", "FIDO2 Information"),
        ("card.led_config", "LED Configuration"),
        ("card.security_status", "Security Status"),
        
        ("device.serial", "Serial Number"),
        ("device.firmware", "Firmware Version"),
        ("device.vid_pid", "VID:PID"),
        ("device.product_name", "Product Name"),
        ("device.flash_memory", "Flash Memory"),
        
        ("fido.aaguid", "AAGUID"),
        ("fido.versions", "FIDO Versions"),
        ("fido.pin_set", "PIN Set"),
        ("fido.resident_keys", "Resident Keys"),
        ("fido.min_pin_length", "Min PIN Length"),
        ("fido.remaining_credentials", "Remaining Credentials"),
        ("fido.pin_set_label", "PIN Set"),
        ("status.not_set", "Not Set"),
        ("status.supported", "Supported"),
        ("status.not_supported", "Not Supported"),
        
        ("led.gpio_pin", "LED GPIO Pin"),
        ("led.brightness", "LED Brightness"),
        ("led.touch_timeout", "Presence Touch Timeout"),
        ("led.dimmable", "LED Dimmable"),
        ("led.steady_mode", "LED Steady Mode"),
        ("led.fido_mode_notice", "Information is not available in Fido only communication mode."),
        
        ("security.boot_mode", "Boot Mode"),
        ("security.secure_boot", "Secure Boot"),
        ("security.development", "Development"),
        ("security.debug_interface", "Debug Interface"),
        ("security.readout_locked", "Read-out Locked"),
        ("security.debug_enabled", "Debug Enabled"),
        ("security.secure_lock", "Secure Lock"),
        ("security.acknowledged", "Acknowledged"),
        ("security.pending", "Pending"),
        
        ("common.yes", "Yes"),
        ("common.no", "No"),
        ("common.on", "On"),
        ("common.off", "Off"),
        ("common.set", "Set"),
        ("action.save", "Save"),
        ("action.cancel", "Cancel"),
        ("action.delete", "Delete"),
        ("action.edit", "Edit"),
        ("action.add", "Add"),
        
        ("notification.success", "Success"),
        ("notification.error", "Error"),
        ("notification.warning", "Warning"),
        ("notification.info", "Information"),
        
        // About 视图
        ("about.title", "About"),
        ("about.description", "Information about the application and its development."),
        ("about.tagline", "An open source commissioning tool for Pico FIDO security keys. Developed with Rust and GPUI."),
        ("about.code_by", "Code By:"),
        ("about.copyright", "Copyright:"),
        
        // Configuration 视图
        ("config.title", "Configuration"),
        ("config.description", "Configure device settings and USB identity."),
        ("config.vendor_preset", "Vendor Preset"),
        ("config.vendor_id", "Vendor ID (HEX)"),
        ("config.product_id", "Product ID (HEX)"),
        ("config.led_settings", "LED Settings"),
        ("config.led_gpio", "LED GPIO Pin"),
        ("config.led_driver", "LED Driver"),
        ("config.led_brightness", "LED Brightness"),
        ("config.touch_timeout", "Touch Timeout (seconds)"),
        ("config.led_dimmable", "LED Dimmable"),
        ("config.led_steady", "LED Steady Mode"),
        ("config.power_cycle", "Power Cycle on Reset"),
        ("config.enable_secp256k1", "Enable SECP256K1"),
        ("config.apply_changes", "Apply Changes"),
        ("config.applying", "Applying Configuration"),
        ("config.success", "Configuration applied successfully."),
        ("config.failed", "Failed to apply configuration"),
        ("config.auth_required", "Authentication Required"),
        ("config.enter_pin", "Enter your device PIN to apply changes."),
        ("action.confirm", "Confirm"),
        ("action.unlock", "Unlock"),
        
        // Passkeys 视图
        ("passkeys.title", "Passkeys"),
        ("passkeys.description", "Manage your FIDO2 credentials and PIN settings."),
        ("passkeys.unlock_storage", "Unlock Storage"),
        ("passkeys.lock_storage", "Lock Storage"),
        ("passkeys.enter_pin", "Enter your device PIN to view saved passkeys"),
        ("passkeys.storage_unlocked", "Storage unlocked successfully."),
        ("passkeys.storage_locked", "Storage locked."),
        ("passkeys.no_credentials", "No credentials stored"),
        ("passkeys.rp_id", "Relying Party"),
        ("passkeys.user", "User"),
        ("passkeys.created", "Created"),
        ("passkeys.delete", "Delete Passkey"),
        ("passkeys.delete_confirm", "Delete Passkey"),
        ("passkeys.delete_confirm_msg", "Are you sure you want to delete the passkey for {0}?"),
        ("passkeys.change_pin", "Change PIN"),
        ("passkeys.setup_pin", "Set PIN"),
        ("passkeys.current_pin", "Current PIN"),
        ("passkeys.new_pin", "New PIN"),
        ("passkeys.confirm_pin", "Confirm New PIN"),
        ("passkeys.pin_changed", "PIN changed successfully."),
        ("passkeys.pin_setup", "PIN configured successfully."),
        ("passkeys.pin_mismatch", "PINs do not match"),
        ("passkeys.pin_min_length", "Minimum PIN Length"),
        ("passkeys.update_min_length", "Update Minimum PIN Length"),
        ("passkeys.min_length_updated", "Minimum length updated to {0}."),
        
        // Security 视图
        ("security_view.title", "Secure Boot"),
        ("security_view.description", "Permanently lock this device to the current firmware vendor."),
        ("security_view.feature_unstable", "Feature Unstable"),
        ("security_view.disabled_warning", "This feature is currently under work and disabled for safety."),
        ("security_view.lock_settings", "Lock Settings"),
        ("security_view.enable_secure_boot", "Enable Secure Boot"),
        ("security_view.enable_secure_boot_desc", "Verifies firmware signature on startup"),
        ("security_view.secure_lock", "Secure Lock"),
        ("security_view.secure_lock_desc", "Prevents reading key material via debug ports"),
        ("security_view.understand_risks", "I understand the risks of bricking my device."),
        ("security_view.lock_device", "Permanently Lock Device"),
        
        // 通用对话框
        ("common.loading", "Loading..."),
        ("common.success", "Success"),
        ("common.error", "Error"),
        ("common.warning", "Warning"),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect()
}

/// 中文翻译
fn load_chinese_translations() -> HashMap<String, String> {
    [
        ("app.name", "PicoForge"),
        ("action.refresh", "刷新"),
        ("action.expand", "展开"),
        ("action.collapse", "收起"),
        ("device.no_device", "未连接设备"),
        ("common.not_available", "不可用"),
        
        ("menu.home", "主页"),
        ("menu.passkeys", "通行密钥"),
        ("menu.configuration", "配置"),
        ("menu.security", "安全"),
        ("menu.about", "关于"),
        ("menu.logs", "日志"),
        
        ("device.status", "设备状态"),
        ("device.status.online", "在线"),
        ("device.status.online_fido", "在线 - Fido 模式"),
        ("device.status.error", "错误"),
        ("device.status.offline", "离线"),
        
        ("home.title", "设备概览"),
        ("home.description", "快速查看您的设备状态和规格。"),
        
        ("card.device_info", "设备信息"),
        ("card.fido2_info", "FIDO2 信息"),
        ("card.led_config", "LED 配置"),
        ("card.security_status", "安全状态"),
        
        ("device.serial", "序列号"),
        ("device.firmware", "固件版本"),
        ("device.vid_pid", "VID:PID"),
        ("device.product_name", "产品名称"),
        ("device.flash_memory", "闪存"),
        
        ("fido.aaguid", "AAGUID"),
        ("fido.versions", "FIDO 版本"),
        ("fido.pin_set", "PIN 已设置"),
        ("fido.resident_keys", "驻留密钥"),
        ("fido.min_pin_length", "最小 PIN 长度"),
        ("fido.remaining_credentials", "剩余凭据"),
        ("fido.pin_set_label", "PIN 设置"),
        ("status.not_set", "未设置"),
        ("status.supported", "支持"),
        ("status.not_supported", "不支持"),
        
        ("led.gpio_pin", "LED GPIO 引脚"),
        ("led.brightness", "LED 亮度"),
        ("led.touch_timeout", "存在触摸超时"),
        ("led.dimmable", "LED 可调光"),
        ("led.steady_mode", "LED 常亮模式"),
        ("led.fido_mode_notice", "在仅 Fido 通信模式下无法获取此信息。"),
        
        ("security.boot_mode", "启动模式"),
        ("security.secure_boot", "安全启动"),
        ("security.development", "开发模式"),
        ("security.debug_interface", "调试接口"),
        ("security.readout_locked", "读取锁定"),
        ("security.debug_enabled", "调试已启用"),
        ("security.secure_lock", "安全锁定"),
        ("security.acknowledged", "已确认"),
        ("security.pending", "待处理"),
        
        ("common.yes", "是"),
        ("common.no", "否"),
        ("common.on", "开"),
        ("common.off", "关"),
        ("common.set", "已设置"),
        ("action.save", "保存"),
        ("action.cancel", "取消"),
        ("action.delete", "删除"),
        ("action.edit", "编辑"),
        ("action.add", "添加"),
        
        ("notification.success", "成功"),
        ("notification.error", "错误"),
        ("notification.warning", "警告"),
        ("notification.info", "信息"),
        
        // About 视图
        ("about.title", "关于"),
        ("about.description", "关于应用程序及其开发的信息。"),
        ("about.tagline", "一个开源的 Pico FIDO 安全密钥配置工具。使用 Rust 和 GPUI 开发。"),
        ("about.code_by", "代码作者："),
        ("about.copyright", "版权所有："),
        
        // Configuration 视图
        ("config.title", "配置"),
        ("config.description", "配置设备设置和 USB 标识。"),
        ("config.vendor_preset", "供应商预设"),
        ("config.vendor_id", "供应商 ID (HEX)"),
        ("config.product_id", "产品 ID (HEX)"),
        ("config.led_settings", "LED 设置"),
        ("config.led_gpio", "LED GPIO 引脚"),
        ("config.led_driver", "LED 驱动器"),
        ("config.led_brightness", "LED 亮度"),
        ("config.touch_timeout", "触摸超时 (秒)"),
        ("config.led_dimmable", "LED 可调光"),
        ("config.led_steady", "LED 常亮模式"),
        ("config.power_cycle", "重置时重新上电"),
        ("config.enable_secp256k1", "启用 SECP256K1"),
        ("config.apply_changes", "应用更改"),
        ("config.applying", "正在应用配置"),
        ("config.success", "配置已成功应用。"),
        ("config.failed", "应用配置失败"),
        ("config.auth_required", "需要认证"),
        ("config.enter_pin", "输入您的设备 PIN 以应用更改。"),
        ("action.confirm", "确认"),
        ("action.unlock", "解锁"),
        
        // Passkeys 视图
        ("passkeys.title", "通行密钥"),
        ("passkeys.description", "管理您的 FIDO2 凭据和 PIN 设置。"),
        ("passkeys.unlock_storage", "解锁存储"),
        ("passkeys.lock_storage", "锁定存储"),
        ("passkeys.enter_pin", "输入您的设备 PIN 以查看保存的通行密钥"),
        ("passkeys.storage_unlocked", "存储已成功解锁。"),
        ("passkeys.storage_locked", "存储已锁定。"),
        ("passkeys.no_credentials", "没有存储的凭据"),
        ("passkeys.rp_id", "依赖方"),
        ("passkeys.user", "用户"),
        ("passkeys.created", "创建时间"),
        ("passkeys.delete", "删除通行密钥"),
        ("passkeys.delete_confirm", "删除通行密钥"),
        ("passkeys.delete_confirm_msg", "您确定要删除 {0} 的通行密钥吗？"),
        ("passkeys.change_pin", "更改 PIN"),
        ("passkeys.setup_pin", "设置 PIN"),
        ("passkeys.current_pin", "当前 PIN"),
        ("passkeys.new_pin", "新 PIN"),
        ("passkeys.confirm_pin", "确认新 PIN"),
        ("passkeys.pin_changed", "PIN 已成功更改。"),
        ("passkeys.pin_setup", "PIN 已成功配置。"),
        ("passkeys.pin_mismatch", "PIN 不匹配"),
        ("passkeys.pin_min_length", "最小 PIN 长度"),
        ("passkeys.update_min_length", "更新最小 PIN 长度"),
        ("passkeys.min_length_updated", "最小长度已更新为 {0}。"),
        
        // Security 视图
        ("security_view.title", "安全启动"),
        ("security_view.description", "将此设备永久锁定到当前固件供应商。"),
        ("security_view.feature_unstable", "功能不稳定"),
        ("security_view.disabled_warning", "此功能目前正在开发中，为确保安全已禁用。"),
        ("security_view.lock_settings", "锁定设置"),
        ("security_view.enable_secure_boot", "启用安全启动"),
        ("security_view.enable_secure_boot_desc", "启动时验证固件签名"),
        ("security_view.secure_lock", "安全锁定"),
        ("security_view.secure_lock_desc", "防止通过调试端口读取密钥材料"),
        ("security_view.understand_risks", "我了解设备变砖的风险。"),
        ("security_view.lock_device", "永久锁定设备"),
        
        // Passkeys 视图 - 额外翻译
        ("action.update", "更新"),
        ("passkeys.set_min_pin_length_desc", "设置允许的最小 PIN 长度（4-63 个字符），并输入满足此要求的新 PIN。"),
        ("passkeys.new_pin_min_chars", "新 PIN（最少 {0} 个字符）"),
        ("passkeys.pin_is_set", "PIN 已设置"),
        ("passkeys.no_pin_configured", "未配置 PIN"),
        ("passkeys.current_min_length", "当前：{0} 个字符"),
        ("passkeys.unlock_to_view", "解锁您的设备以查看和管理通行密钥。"),
        ("passkeys.stored_passkeys", "存储的通行密钥"),
        ("passkeys.manage_credentials_desc", "查看和管理您的驻留凭据"),
        ("passkeys.unlocked", "已解锁"),
        ("passkeys.credentials_stored", "{0} 个凭据已存储"),
        
        // 通用对话框
        ("common.loading", "加载中..."),
        ("common.success", "成功"),
        ("common.error", "错误"),
        ("common.warning", "警告"),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect()
}

/// 便捷的翻译函数
pub fn t(key: TranslationKey) -> &'static str {
    Translator::global().t(key)
}

/// 带参数的便捷翻译函数
pub fn t_with_args(key: TranslationKey, args: &[&str]) -> String {
    Translator::global().t_with_args(key, args)
}
