use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use vb6runtime::layout::{self, renderer::{Renderer, TauriRenderer}};
use vb6runtime::layout::model::{LayoutControlType, LayoutLeaf, LayoutNode, LayoutPosition, LayoutSize};

fn make_label_leaf(index: i32, value: &str) -> LayoutLeaf {
    let idx = index as f32;
    LayoutLeaf {
        name: format!("Label{}", index),
        control_type: LayoutControlType::Label,
        index,
        position: LayoutPosition {
            left: (idx * 200.0) % 500.0,
            top: ((index / 5) as f32) * 200.0,
        },
        size: LayoutSize { width: 500.0, height: 200.0 },
        style: Default::default(),
        value: Some(value.to_string()),
        visible: true,
        enabled: true,
    }
}

fn make_button_leaf(index: i32) -> LayoutLeaf {
    let idx = index as f32;
    LayoutLeaf {
        name: format!("Button{}", index),
        control_type: LayoutControlType::CommandButton,
        index,
        position: LayoutPosition {
            left: (idx * 300.0) % 600.0,
            top: 100.0 + ((index / 4) as f32) * 200.0,
        },
        size: LayoutSize { width: 400.0, height: 200.0 },
        style: Default::default(),
        value: Some(format!("Button {}", index)),
        visible: true,
        enabled: true,
    }
}

fn make_textbox_leaf(index: i32) -> LayoutLeaf {
    let idx = index as f32;
    LayoutLeaf {
        name: format!("Text{}", index),
        control_type: LayoutControlType::TextBox,
        index,
        position: LayoutPosition {
            left: (idx * 250.0) % 550.0,
            top: 150.0 + ((index / 3) as f32) * 200.0,
        },
        size: LayoutSize { width: 600.0, height: 250.0 },
        style: Default::default(),
        value: Some(format!("Text {}", index)),
        visible: true,
        enabled: true,
    }
}

fn build_form_with_labels(count: usize) -> layout::model::LayoutForm {
    let mut children = Vec::with_capacity(count);
    for i in 0..count {
        children.push(LayoutNode::Leaf(make_label_leaf(
            i as i32,
            &format!("Label text {}", i),
        )));
    }
    layout::model::LayoutForm {
        name: "BenchmarkForm".to_string(),
        control_type: LayoutControlType::Form,
        index: 0,
        position: LayoutPosition::default(),
        size: LayoutSize { width: 1000.0, height: 800.0 },
        style: Default::default(),
        root_node: LayoutNode::Container(layout::model::LayoutContainer {
            name: "BenchmarkForm".to_string(),
            control_type: LayoutControlType::Form,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize { width: 1000.0, height: 800.0 },
            style: Default::default(),
            children,
            caption: Some("Benchmark Form".to_string()),
            visible: true,
            enabled: true,
            current_value: None,
        }),
        caption: "Benchmark Form".to_string(),
        visible: true,
        enabled: true,
        current_value: None,
        snapshot: None,
        render_id: 0,
    }
}

fn build_form_with_mixed_controls(count: usize) -> layout::model::LayoutForm {
    let mut children = Vec::with_capacity(count);
    let labels = count / 3;
    let buttons = count / 3;
    let textboxes = count - labels - buttons;
    for i in 0..labels {
        children.push(LayoutNode::Leaf(make_label_leaf(
            i as i32,
            &format!("Label {}", i),
        )));
    }
    for i in 0..buttons {
        children.push(LayoutNode::Leaf(make_button_leaf(i as i32)));
    }
    for i in 0..textboxes {
        children.push(LayoutNode::Leaf(make_textbox_leaf(i as i32)));
    }
    layout::model::LayoutForm {
        name: "MixedForm".to_string(),
        control_type: LayoutControlType::Form,
        index: 0,
        position: LayoutPosition::default(),
        size: LayoutSize { width: 1200.0, height: 900.0 },
        style: Default::default(),
        root_node: LayoutNode::Container(layout::model::LayoutContainer {
            name: "MixedForm".to_string(),
            control_type: LayoutControlType::Form,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize { width: 1200.0, height: 900.0 },
            style: Default::default(),
            children,
            caption: Some("Mixed Control Form".to_string()),
            visible: true,
            enabled: true,
            current_value: None,
        }),
        caption: "Mixed Control Form".to_string(),
        visible: true,
        enabled: true,
        current_value: None,
        snapshot: None,
        render_id: 0,
    }
}

fn benchmark_render_10_controls(c: &mut Criterion) {
    let form = build_form_with_labels(10);
    let renderer = TauriRenderer::new(false);

    c.bench_function("render_10_controls_labels", |b| {
        b.iter(|| {
            let f = form.clone();
            black_box(renderer.render_node(black_box(&f.root_node)))
        })
    });
}

fn benchmark_render_100_controls(c: &mut Criterion) {
    let form = build_form_with_mixed_controls(100);
    let renderer = TauriRenderer::new(false);

    c.bench_function("render_100_controls_mixed", |b| {
        b.iter(|| {
            let f = form.clone();
            black_box(renderer.render_node(black_box(&f.root_node)))
        })
    });
}

fn benchmark_render_500_controls(c: &mut Criterion) {
    let form = build_form_with_labels(500);
    let renderer = TauriRenderer::new(false);

    c.bench_function("render_500_controls_labels", |b| {
        b.iter(|| {
            let f = form.clone();
            black_box(renderer.render_node(black_box(&f.root_node)))
        })
    });
}

fn benchmark_diff_vs_full_render(c: &mut Criterion) {
    let renderer = TauriRenderer::new(false);

    c.bench_function("full_render_50_controls", |b| {
        let form = build_form_with_mixed_controls(50);
        b.iter(|| {
            let f = form.clone();
            black_box(renderer.render_node(black_box(&f.root_node)))
        })
    });

    c.bench_function("diff_render_50_controls_no_change", |b| {
        let form = build_form_with_mixed_controls(50);
        // First render to capture snapshot
        let _ = renderer.render_node(&form.root_node);
        b.iter(|| {
            let mut f = form.clone();
            f.snapshot = Some(f.root_node.to_snapshot());
            let diff = f.snapshot.as_ref().map(|snap| {
                layout::DiffEngine::compute_diff(snap, &f.root_node)
            });
            black_box(renderer.render_node_with_diff(
                black_box(&f.root_node),
                black_box(diff.as_ref()),
            ))
        })
    });

    c.bench_function("diff_render_50_controls_value_changed", |b| {
        let form = build_form_with_mixed_controls(50);
        b.iter(|| {
            let mut f = form.clone();
            f.snapshot = Some(f.root_node.to_snapshot());
            // Mutate one leaf
            if let LayoutNode::Container(ref mut root) = f.root_node
                && let Some(LayoutNode::Leaf(leaf)) = root.children.first_mut()
            {
                leaf.value = Some("MUTATED".to_string());
            }
            let diff = f.snapshot.as_ref().map(|snap| {
                layout::DiffEngine::compute_diff(snap, &f.root_node)
            });
            black_box(renderer.render_node_with_diff(
                black_box(&f.root_node),
                black_box(diff.as_ref()),
            ))
        })
    });
}

criterion_group!(
    benches,
    benchmark_render_10_controls,
    benchmark_render_100_controls,
    benchmark_render_500_controls,
    benchmark_diff_vs_full_render,
);
criterion_main!(benches);
