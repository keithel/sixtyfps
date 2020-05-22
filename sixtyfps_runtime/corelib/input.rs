/*! Module handling mouse events

TODO: Keyboard events
*/

use crate::abi::datastructures::{ComponentRef, MouseEvent};
use euclid::default::Vector2D;

pub fn process_mouse_event(component: ComponentRef<'_>, event: MouseEvent) {
    let offset = Vector2D::new(0., 0.);

    crate::abi::datastructures::visit_items(
        component,
        |item, offset| {
            let geom = item.geometry();
            let geom = geom.translate(*offset);

            if geom.contains(event.pos) {
                let mut event2 = event.clone();
                event2.pos -= geom.origin.to_vector();
                item.input_event(event2, component);
            }

            geom.origin.to_vector()
        },
        offset,
    );
}