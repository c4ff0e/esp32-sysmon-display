pub enum FrameKind {
    Cpu,
    Gpu,
    GpuAndCpu,
}
pub enum RenderDecision{
    Unsupported(FrameKind),
    Full,
}