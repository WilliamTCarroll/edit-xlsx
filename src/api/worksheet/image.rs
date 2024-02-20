use std::path::Path;
use crate::api::cell::location::LocationRange;
use crate::api::worksheet::Sheet;
use crate::xml::drawings::Drawings;
use crate::xml::relationships::Relationships;

pub(crate) trait _Image {
    fn add_background<P: AsRef<Path>>(&self, filename: &P) -> u32;
    fn add_drawing_image<L: LocationRange,P: AsRef<Path>>(&mut self, loc: L, filename: &P) -> u32;
}

impl _Image for Sheet {
    fn add_background<P: AsRef<Path>>(&self, filename: &P) -> u32 {
        let mut worksheets_rel = self.worksheets_rel.borrow_mut();
        if let None = worksheets_rel.get_mut(&self.id) {
            worksheets_rel.insert(self.id, Relationships::default());
        }
        let worksheet_rel = worksheets_rel.get_mut(&self.id).unwrap();
        self.content_types.borrow_mut().add_png();
        let image_id = self.medias.borrow_mut().add_media(filename);
        let r_id = worksheet_rel.next_id();
        worksheet_rel.add_image(r_id, image_id);
        r_id
    }

    fn add_drawing_image<L: LocationRange,P: AsRef<Path>>(&mut self, loc: L, filename: &P) -> u32 {
        let image_id = self.medias.borrow_mut().add_media(filename);
        self.content_types.borrow_mut().add_drawing(image_id);
        let drawings_rel = &mut self.drawings_rel;
        if let None = drawings_rel.get_mut(&image_id) {
            drawings_rel.insert(image_id, Relationships::default());
        }
        let drawing_rel = self.drawings_rel.get_mut(&image_id).unwrap();
        let r_id = drawing_rel.next_id();
        drawing_rel.add_image(r_id, image_id);

        let drawings = &mut self.drawings;
        if let None = drawings.get_mut(&image_id) {
            drawings.insert(image_id, Drawings::default());
        }
        let drawing = drawings.get_mut(&image_id).unwrap();
        drawing.add_drawing(loc, r_id);

        let mut worksheets_rel = self.worksheets_rel.borrow_mut();
        if let None = worksheets_rel.get_mut(&self.id) {
            worksheets_rel.insert(self.id, Relationships::default());
        }
        let worksheet_rel = worksheets_rel.get_mut(&self.id).unwrap();
        worksheet_rel.add_drawing(r_id, image_id);
        // .get(&0).take().unwrap_or_default();
        r_id
    }
}