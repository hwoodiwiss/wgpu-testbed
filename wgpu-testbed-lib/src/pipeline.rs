pub trait Bindable {
    fn layout_entries() -> Vec<wgpu::BindGroupLayoutEntry>;
    fn layout_entries() -> Vec<wgpu::BindGroupEntry>;
}
