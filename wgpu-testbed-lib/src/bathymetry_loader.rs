use std::path::Path;

use crate::{file_reader::FileReader, vertex::Vertex};
use anyhow::Result;
use serde::Deserialize;
use std::f64;
use wgpu::util::DeviceExt;

#[derive(Debug, Deserialize)]
pub struct BathyPoint {
    #[serde(rename(deserialize = "Lat (DD)"))]
    pub latitude: f64,
    #[serde(rename(deserialize = "Long (DD)"))]
    pub longitude: f64,
    #[serde(rename(deserialize = "Depth"))]
    pub depth: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BathyVertex {
    position: [f64; 3],
    padding: u32,
}

impl Vertex for BathyVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float64x3,
                offset: 0,
                shader_location: 0,
            }],
        }
    }
}

pub struct BathyMesh {
    name: String,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_elements: u32,
}

struct BathymetryLoader {}

impl BathymetryLoader {
    fn latitude_degrees_to_metres(latitude: f64) -> f64 {
        let y = (((90f64 + latitude) * f64::consts::PI) / 360f64).tan().ln()
            / (f64::consts::PI / 180f64);
        (y * 20037508.34f64) / 180f64
    }

    fn longitude_degrees_to_metres(longitude: f64) -> f64 {
        (longitude * 20037508.34f64) / 180.0f64
    }

    fn centre_on_origin(points: &Vec<BathyPoint>) -> Vec<BathyPoint> {
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;

        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for point in points {
            println!("{:?}", point);
            if point.longitude < min_x {
                min_x = point.longitude;
            }

            if max_x < point.longitude {
                max_x = point.longitude;
            }

            if point.latitude < min_y {
                min_y = point.latitude;
            }

            if max_y < point.latitude {
                max_y = point.latitude;
            }
        }

        points
            .into_iter()
            .map(|point| BathyPoint {
                latitude: ((max_y - min_y) / 2f64) - (point.latitude - min_y),
                longitude: ((max_x - min_x) / 2f64) - (point.longitude - min_x),
                depth: point.depth,
            })
            .collect::<Vec<_>>()
    }

    fn calculate_indices(points: &Vec<BathyPoint>) -> Vec<u32> {
        return vec![0u32];
    }

    pub async fn load<P: AsRef<Path>>(&self, device: &wgpu::Device, path: P) -> Result<BathyMesh> {
        let path = path.as_ref();
        let obj_data =
            FileReader::read_file(path.to_str().expect("Could not convert model path to &str"))
                .await;

        let mut csv_reader = csv::Reader::from_reader(&obj_data[..]);
        let mut points = Vec::<BathyPoint>::new();

        for point_res in csv_reader.deserialize() {
            let mut point: BathyPoint = point_res?;
            point.latitude = Self::latitude_degrees_to_metres(point.latitude);
            point.longitude = Self::longitude_degrees_to_metres(point.longitude);
            points.push(point);
        }

        let points = Self::centre_on_origin(&points);

        let indices = Self::calculate_indices(&points);

        let vertices = points
            .into_iter()
            .map(|point| BathyVertex {
                position: [point.longitude, point.latitude, point.depth],
                padding: 0u32,
            })
            .collect::<Vec<_>>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Bathy Surface Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices[..]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Bathy Surface Vertex Buffer"),
            contents: bytemuck::cast_slice(&indices[..]),
            usage: wgpu::BufferUsages::INDEX,
        });

        Ok(BathyMesh {
            name: "Bathy Surface".to_owned(),
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
        })
    }
}
