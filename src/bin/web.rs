use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut file_name = use_signal(|| String::new());
    let mut is_processing = use_signal(|| false);
    let mut error_message = use_signal(|| String::new());
    let mut spline_step = use_signal(|| 20u32);

    rsx! {
        div {
            class: "container",
            style: "max-width: 800px; margin: 0 auto; padding: 20px; font-family: system-ui, -apple-system, sans-serif;",
            h1 {
                style: "color: #2563eb; margin-bottom: 10px;",
                "DXF to ELMT Converter"
            }
            p {
                style: "color: #6b7280; margin-bottom: 30px;",
                "Convert DXF files to QElectroTech ELMT format"
            }

            div {
                style: "background: #f9fafb; border: 2px dashed #d1d5db; border-radius: 8px; padding: 30px; text-align: center; margin-bottom: 20px;",
                label {
                    style: "display: block; margin-bottom: 10px; font-weight: 500; color: #374151;",
                    "Select DXF File:"
                }
                input {
                    r#type: "file",
                    accept: ".dxf",
                    onchange: move |evt| {
                        // File handling will be implemented with proper WASM file API
                        let value = evt.value();
                        if !value.is_empty() {
                            // Extract filename from path
                            let path = value.replace('\\', "/");
                            if let Some(name) = path.split('/').last() {
                                file_name.set(name.to_string());
                            }
                        }
                    },
                    style: "margin: 10px 0; padding: 8px; border: 1px solid #d1d5db; border-radius: 4px; width: 100%; max-width: 400px;"
                }
                if !file_name().is_empty() {
                    p {
                        style: "margin-top: 10px; color: #059669; font-weight: 500;",
                        "Selected: {file_name()}"
                    }
                }
            }

            div {
                style: "margin-bottom: 20px;",
                label {
                    style: "display: block; margin-bottom: 5px; font-weight: 500; color: #374151;",
                    "Spline Step (resolution):"
                }
                input {
                    r#type: "number",
                    value: "{spline_step()}",
                    min: "1",
                    max: "100",
                    oninput: move |evt| {
                        if let Ok(value) = evt.value().parse::<u32>() {
                            spline_step.set(value);
                        }
                    },
                    style: "padding: 8px; border: 1px solid #d1d5db; border-radius: 4px; width: 100px;"
                }
                p {
                    style: "margin-top: 5px; color: #6b7280; font-size: 0.875rem;",
                    "Higher values = greater resolution (default: 20)"
                }
            }

            button {
                onclick: move |_| {
                    if file_name().is_empty() {
                        error_message.set("Please select a file first".to_string());
                        return;
                    }
                    is_processing.set(true);
                    error_message.set(String::new());

                    // Note: File conversion in browser requires:
                    // 1. Reading file content via FileReader API (WASM)
                    // 2. Processing the DXF file in memory
                    // 3. Generating and downloading the ELMT file
                    // This is a placeholder - full implementation would require
                    // additional WASM bindings for file I/O
                    error_message.set("Web conversion is in development. For now, please use the CLI version: dxf2elmt <file.dxf>".to_string());
                    is_processing.set(false);
                },
                disabled: is_processing() || file_name().is_empty(),
                style: "background: #2563eb; color: white; border: none; padding: 12px 24px; border-radius: 6px; font-size: 16px; font-weight: 500; cursor: pointer; width: 100%; max-width: 400px;",
                if is_processing() {
                    "Processing..."
                } else {
                    "Convert to ELMT"
                }
            }

            if !error_message().is_empty() {
                div {
                    style: "margin-top: 20px; padding: 12px; background: #fef2f2; border: 1px solid #fecaca; border-radius: 6px; color: #991b1b;",
                    "Error: {error_message()}"
                }
            }

            div {
                style: "margin-top: 30px; padding: 20px; background: #f0f9ff; border: 1px solid #93c5fd; border-radius: 8px;",
                h3 {
                    style: "color: #1e40af; margin-bottom: 10px;",
                    "ℹ️ Web Interface"
                }
                p {
                    style: "color: #1e40af;",
                    "The web interface is currently in development. "
                    "For now, please use the CLI version:"
                }
                code {
                    style: "display: block; margin-top: 10px; padding: 10px; background: white; border-radius: 4px; font-family: monospace;",
                    "dxf2elmt <file.dxf> [options]"
                }
            }
        }
    }
}

#[component]
fn StatItem(label: String, value: u32) -> Element {
    rsx! {
        div {
            style: "padding: 8px; background: white; border-radius: 4px;",
            div {
                style: "font-size: 0.875rem; color: #6b7280;",
                "{label}:"
            }
            div {
                style: "font-size: 1.25rem; font-weight: 600; color: #166534;",
                "{value}"
            }
        }
    }
}

