use nanoserde::{DeJson, SerJson};

#[derive(Clone, Debug, Default, DeJson, SerJson)]
struct Button {
    id: String,
    image_id: String,
    label: String,
    border_color: Option<String>,
    background_color: Option<String>,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
struct Grid {
    rows: usize,
    columns: usize,
    order: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
struct Image {
    id: String,
    url: String,
    width: usize,
    height: usize,
    content_type: String,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct Board {
    format: String,
    id: String,
    locale: String,
    url: String,
    description_html: String,
    buttons: Vec<Button>,
    grid: Grid,
    images: Vec<Image>,
}

#[test]
fn test_obf_board() {
    let json = r#"{
        "format": "open-board-0.1",
        "id": "1",
        "locale": "en",
        "url": "https://example.com/boards/123",
        "name": "Example Board",
        "description_html": "This is just a <b>simple</b> example board I put together.",
        "buttons": [
            {
                "id": "1",
                "image_id": "2",
                "label": "happy",
                "border_color": "rgb(0, 0, 55)",
                "background_color": "rgba(200, 255, 255, 0.2)"
            },
            {
                "id": "2",
                "label": "drinks",
                "image_id": "2"
            }
        ],
        "grid": {
            "rows": 1,
            "columns": 2,
            "order": [
                ["1","2"]
            ]
        },
        "images": [
            {
                "id": "1",
                "url": "https://example.com/happy.png",
                "width": 1024,
                "height": 768,
                "content_type": "image/png"
            }
        ]
    }"#;

    let board: Board = DeJson::deserialize_json(json).unwrap();
    assert_eq!(board.format, "open-board-0.1");
    assert_eq!(board.id, "1");
    assert_eq!(board.locale, "en");
    assert_eq!(board.url, "https://example.com/boards/123");
    assert_eq!(board.buttons[0].background_color, Some("rgba(200, 255, 255, 0.2)".to_string()));
    assert_eq!(board.buttons[1].background_color, None);
    assert_eq!(board.grid.order, vec![vec!["1".to_string(), "2".to_string()]]);
    assert_eq!(board.images[0].url, "https://example.com/happy.png");
}
