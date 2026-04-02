use plotters::prelude::*;

// matplotlib tab20 palette — enough for 15 combos (5 singles + 10 pairs)
const PALETTE: &[RGBColor] = &[
    RGBColor(31, 119, 180),  RGBColor(255, 127, 14),  RGBColor(44, 160, 44),
    RGBColor(214, 39, 40),   RGBColor(148, 103, 189), RGBColor(140, 86, 75),
    RGBColor(227, 119, 194), RGBColor(127, 127, 127), RGBColor(188, 189, 34),
    RGBColor(23, 190, 207),  RGBColor(174, 199, 232), RGBColor(255, 187, 120),
    RGBColor(152, 223, 138), RGBColor(255, 152, 150), RGBColor(197, 176, 213),
];

pub fn plot(points: &[(usize, String, f64)]) {
    let mut combos: Vec<&String> = Vec::new();
    for (_, combo, _) in points {
        if !combos.contains(&combo) {
            combos.push(combo);
        }
    }

    let max_len   = points.iter().map(|(l, _, _)| *l).max().unwrap_or(1);
    let min_len   = points.iter().map(|(l, _, _)| *l).min().unwrap_or(0);
    let max_steps = points.iter().map(|(_, _, s)| *s).fold(0f64, f64::max);

    let root = BitMapBackend::new("plot.png", (1200, 700)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Extend x range past max_len to give end-of-line labels room to breathe
    let x_padding = (max_len - min_len) / 4;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sorting combinations: length vs avg steps", ("sans-serif", 20))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(70)
        .build_cartesian_2d(min_len..(max_len + x_padding), 0f64..max_steps * 1.05)
        .unwrap();

    chart.configure_mesh()
        .x_desc("List length")
        .y_desc("Avg steps")
        .light_line_style(WHITE)   // hide minor grid lines
        .bold_line_style(RGBColor(220, 220, 220))  // subtle axis grid only
        .draw().unwrap();

    for (ci, combo) in combos.iter().enumerate() {
        let color = PALETTE[ci % PALETTE.len()];
        let data: Vec<(usize, f64)> = points.iter()
            .filter(|(_, c, _)| c == *combo)
            .map(|(l, _, s)| (*l, *s))
            .collect();

        // Draw the line (no legend)
        chart.draw_series(LineSeries::new(data.clone(), color.stroke_width(2))).unwrap();

        // Label at the end of the line
        if let Some(&(x, y)) = data.last() {
            let label_x = x + (max_len - min_len) / 30 + 1;
            chart.draw_series(std::iter::once(Text::new(
                combo.to_string(),
                (label_x, y),
                ("sans-serif", 13).into_font().color(&color),
            ))).unwrap();
        }
    }

    root.present().unwrap();
}
