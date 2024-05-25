/// `${string}.dweb`
pub struct MMID {
    value: String,
}

///`dweb:${string}`
pub struct DwebDeepLink {
    value: String,
}

/**
 * 通讯支持的传输协议
 */
pub struct IpcSupportProtocols {
    cbor: bool,
    protobuf: bool,
    json: bool,
}
