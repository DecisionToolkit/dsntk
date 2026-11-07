use crate::auto_size::AutoSize;
use dsntk_model::{DcBounds, DcPoint, DmnDiagram, DmnEdge, DmnShape};

fn diagram() -> DmnDiagram {
    DmnDiagram {
        id: None,
        name: None,
        documentation: "".to_string(),
        resolution: 0.0,
        diagram_elements: vec![],
        shared_style: None,
        local_style: None,
        size: None,
    }
}

#[test]
fn default_width_and_height_should_work() {
    let auto_size = AutoSize::new();
    let dimension = auto_size.dimension(&diagram());
    assert_eq!(
        "DcDimension { width: 300.0, height: 300.0 }",
        format!("{:?}", dimension)
    );
}

#[test]
fn discovering_from_shape_should_work() {
    let mut auto_size = AutoSize::new();
    auto_size.discover_from_shape(&DmnShape {
        id: None,
        bounds: DcBounds {
            x: 100.0,
            y: 110.0,
            width: 200.0,
            height: 220.0,
        },
        dmn_element_ref: None,
        is_listed_input_data: false,
        decision_service_divider_line: None,
        is_collapsed: false,
        shared_style: None,
        local_style: None,
        label: None,
    });
    let dimension = auto_size.dimension(&diagram());
    assert_eq!(
        "DcDimension { width: 400.0, height: 440.0 }",
        format!("{:?}", dimension)
    );
}

#[test]
fn discovering_from_edge_should_work() {
    let mut auto_size = AutoSize::new();
    auto_size.discover_from_edge(&DmnEdge {
        id: None,
        way_points: vec![DcPoint { x: 10.0, y: 20.0 }, DcPoint { x: 200.0, y: 220.0 }],
        dmn_element_ref: None,
        source_element: None,
        target_element: None,
        shared_style: None,
        local_style: None,
        label: None,
    });
    let dimension = auto_size.dimension(&diagram());
    assert_eq!(
        "DcDimension { width: 210.0, height: 240.0 }",
        format!("{:?}", dimension)
    );
}
