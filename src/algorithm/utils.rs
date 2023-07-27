use super::GraphId;

pub fn escape(input: &str) -> String {
    input.replace('\"', "#quot;")
}

pub fn draw_edge(from: Option<&GraphId>, to: Option<&GraphId>) -> String {
    if let (Some(from), Some(to)) = (from, to) {
        format!("{} --> {}\n", from, to)
    } else {
        "".to_owned()
    }
}

pub fn draw_edge_comment(from: Option<&GraphId>, to: Option<&GraphId>, comment: &str) -> String {
    if let (Some(from), Some(to)) = (from, to) {
        format!("{} -->|\"{}\"| {}\n", from, escape(comment), to)
    } else {
        "".to_owned()
    }
}

pub fn draw_box(id: &GraphId, label: &str) -> String {
    format!("{id}[\"{}\"]\n", escape(label))
}

pub fn draw_rhombus(id: &GraphId, label: &str) -> String {
    format!("{id}{{\"{}\"}}\n", escape(label))
}

pub fn draw_hexagon(id: &GraphId, label: &str) -> String {
    format!("{id}{{{{\"{}\"}}}}\n", escape(label))
}

pub fn draw_dot(id: &GraphId) -> String {
    format!("{id}(( ))\n")
}

