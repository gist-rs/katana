fn main() {
    let svg_data = std::fs::read("./assets/hiragana.svg").unwrap();
    let opt = usvg::Options::default();
    let fontdb = usvg::fontdb::Database::new();
    let tree = usvg::Tree::from_data(&svg_data, &opt, &fontdb).unwrap();

    let xml_opt = usvg::WriteOptions::default();
    let output_svg = tree.to_string(&xml_opt);

    std::fs::write("./assets/hiragana-normalized.svg", output_svg).unwrap();
}
