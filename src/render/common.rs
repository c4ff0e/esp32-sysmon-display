pub enum FrameKind {
    Cpu,
    Gpu,
    GpuAndCpu,
    NoMetrics
}
pub enum RenderDecision {
    Unsupported(FrameKind),
    Full,
    ConnectUsb
}
