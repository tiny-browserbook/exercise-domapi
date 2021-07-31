use cursive::{views::LayerPosition, CbSink};
use std::rc::Rc;

use crate::browser::Browser;

pub struct BrowserAPI {
    ui_cb_sink: Rc<CbSink>,
}

impl BrowserAPI {
    pub fn new(ui_cb_sink: Rc<CbSink>) -> Self {
        Self { ui_cb_sink }
    }

    pub fn rerender(&self) {
        self.ui_cb_sink
            .send(Box::new(move |s: &mut cursive::Cursive| {
                let screen = s.screen_mut();
                let layer: &mut Browser = screen
                    .get_mut(LayerPosition::FromFront(0))
                    .unwrap()
                    .downcast_mut()
                    .unwrap();
                layer.rerender()
            }))
            .unwrap();
    }
}
