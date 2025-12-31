use gtk::prelude::*;
use gtk::{Label, ListBox, ListBoxRow};

pub fn populate_list(list_box: &ListBox, items: &[String]) {
    for item in items {
        let row = ListBoxRow::new();
        let label = Label::new(Some(item));
        label.set_xalign(0.0);
        label.set_margin_start(10);
        label.set_margin_end(10);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        row.set_child(Some(&label));
        list_box.append(&row);
    }
}

pub fn filter_list(list_box: &ListBox, _items: &[String], query: &str) {
    let mut index = 0;
    while let Some(row) = list_box.row_at_index(index) {
        if let Some(label) = row.child().and_then(|w| w.downcast::<Label>().ok()) {
            let text = label.text().to_string();
            let visible = query.is_empty() || text.to_lowercase().contains(query);
            row.set_visible(visible);
        }
        index += 1;
    }
}
