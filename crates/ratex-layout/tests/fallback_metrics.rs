use ratex_layout::{layout, LayoutOptions};
use ratex_parser::parse;
use ratex_types::{color::Color, MathStyle};

fn measured(ch: char) -> Option<(f64, f64, f64)> {
    match ch {
        '漢' => Some((0.73, 0.82, 0.21)),
        'ᚠ' => Some((0.64, 0.70, 0.20)),
        _ => None,
    }
}

#[test]
fn fallback_layout_uses_measured_advance_height_and_depth() {
    let ast = parse(r"\text{漢ᚠ}").unwrap();
    let options = LayoutOptions {
        fallback_metrics: Some(measured),
        ..Default::default()
    };
    let result = layout(&ast, &options);
    assert!((result.width - 1.37).abs() < 1e-9);
    assert!((result.height - 0.82).abs() < 1e-9);
    assert!((result.depth - 0.21).abs() < 1e-9);
}

#[test]
fn declining_resolver_retains_default_metrics() {
    fn decline(_: char) -> Option<(f64, f64, f64)> {
        None
    }
    let ast = parse(r"\text{漢ᚠ}").unwrap();
    let default = layout(&ast, &LayoutOptions::default());
    let declined = layout(
        &ast,
        &LayoutOptions {
            fallback_metrics: Some(decline),
            ..Default::default()
        },
    );
    assert_eq!(declined.width, default.width);
    assert_eq!(declined.height, default.height);
    assert_eq!(declined.depth, default.depth);
}

#[test]
fn known_katex_glyphs_do_not_call_fallback_resolver() {
    fn unexpected(_: char) -> Option<(f64, f64, f64)> {
        panic!("KaTeX glyph must retain its original metrics")
    }
    let ast = parse("x + 2").unwrap();
    let default = layout(&ast, &LayoutOptions::default());
    let result = layout(
        &ast,
        &LayoutOptions {
            fallback_metrics: Some(unexpected),
            ..Default::default()
        },
    );
    assert_eq!(result.width, default.width);
    assert_eq!(result.height, default.height);
    assert_eq!(result.depth, default.depth);
}

#[test]
fn fallback_resolver_survives_derived_style_color_and_tracking_options() {
    let options = LayoutOptions {
        fallback_metrics: Some(measured),
        ..Default::default()
    }
    .with_style(MathStyle::Script)
    .with_color(Color::BLACK)
    .with_inter_glyph_kern(0.02);
    assert_eq!(options.fallback_metrics.unwrap()('漢'), measured('漢'));
    let result = layout(&parse(r"\text{漢}").unwrap(), &options);
    assert!((result.width - 0.73).abs() < 1e-9);
    assert!((result.depth - 0.21).abs() < 1e-9);
}
