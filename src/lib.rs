#![warn(
    clippy::all,
    clippy::pedantic,
)]

pub mod qelmt;
pub mod file_writer;

use anyhow::{Context, Result};
use dxf::entities::EntityType;
use dxf::Drawing;
use qelmt::Definition;
use simple_xml_builder::XMLElement;
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConversionStats {
    pub circles: u32,
    pub lines: u32,
    pub arcs: u32,
    pub splines: u32,
    pub texts: u32,
    pub ellipses: u32,
    pub polylines: u32,
    pub lwpolylines: u32,
    pub solids: u32,
    pub blocks: u32,
    pub unsupported: u32,
    pub elapsed_ms: u128,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConversionResult {
    pub success: bool,
    pub message: String,
    pub stats: Option<ConversionStats>,
    pub xml_content: Option<String>,
}

pub struct ConversionOptions {
    pub spline_step: u32,
    pub verbose: bool,
    pub info: bool,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            spline_step: 20,
            verbose: false,
            info: false,
        }
    }
}

pub fn convert_dxf_file(
    file_path: &Path,
    options: &ConversionOptions,
) -> Result<ConversionResult> {
    let now = Instant::now();
    let friendly_file_name = file_path
        .file_stem()
        .unwrap_or_else(|| file_path.as_os_str())
        .to_string_lossy()
        .to_string();

    // Load DXF file
    let drawing = Drawing::load_file(file_path).context(format!(
        "Failed to load {friendly_file_name}...\n\tMake sure the file is a valid .dxf file.",
    ))?;

    let q_elmt = Definition::new(friendly_file_name.clone(), options.spline_step, &drawing);

    // Initialize counts
    let mut circle_count: u32 = 0;
    let mut line_count: u32 = 0;
    let mut arc_count: u32 = 0;
    let mut spline_count: u32 = 0;
    let mut text_count: u32 = 0;
    let mut ellipse_count: u32 = 0;
    let mut polyline_count: u32 = 0;
    let mut lwpolyline_count: u32 = 0;
    let mut solid_count: u32 = 0;
    let mut block_count: u32 = 0;
    let mut other_count: u32 = 0;

    // Loop through all entities, counting the element types
    drawing.entities().for_each(|e| match e.specific {
        EntityType::Circle(_) => circle_count += 1,
        EntityType::Line(_) => line_count += 1,
        EntityType::Arc(_) => arc_count += 1,
        EntityType::Spline(_) => spline_count += 1,
        EntityType::Text(_) => text_count += 1,
        EntityType::Ellipse(_) => ellipse_count += 1,
        EntityType::Polyline(_) => polyline_count += 1,
        EntityType::LwPolyline(_) => lwpolyline_count += 1,
        EntityType::Solid(_) => solid_count += 1,
        EntityType::Insert(_) => block_count += 1,
        _ => other_count += 1,
    });

    // Generate XML
    let out_xml = XMLElement::from(&q_elmt);
    let xml_content = if options.verbose {
        Some(format!("{}", out_xml))
    } else {
        None
    };

    // Create output file if not verbose
    if !options.verbose {
        let out_file = file_writer::create_file(false, options.info, file_path)?;
        out_xml
            .write(&out_file)
            .context("Failed to write output file.")?;
    }

    let elapsed_ms = now.elapsed().as_millis();

    let stats = ConversionStats {
        circles: circle_count,
        lines: line_count,
        arcs: arc_count,
        splines: spline_count,
        texts: text_count,
        ellipses: ellipse_count,
        polylines: polyline_count,
        lwpolylines: lwpolyline_count,
        solids: solid_count,
        blocks: block_count,
        unsupported: other_count,
        elapsed_ms,
    };

    Ok(ConversionResult {
        success: true,
        message: format!("Successfully converted {}", friendly_file_name),
        stats: Some(stats),
        xml_content,
    })
}

