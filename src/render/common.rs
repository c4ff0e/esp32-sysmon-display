pub enum FrameKind {
    Cpu,
    Gpu,
    GpuAndCpu,
}
pub enum RenderDecision {
    Unsupported(FrameKind),
    Full,
    ConnectUsb,
    NoMetrics
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScreenState{
    ConnectUsb,
    NoMetrics,
    UnsupportedCpu,
    UnsupportedGpu,
    UnsupportedCpuAndGpu,
    Full
}