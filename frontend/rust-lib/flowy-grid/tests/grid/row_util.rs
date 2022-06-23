use crate::grid::script::GridEditorTest;
use flowy_grid::services::row::{CreateRowRevisionBuilder, CreateRowRevisionPayload};
use flowy_grid_data_model::entities::FieldType;
use flowy_grid_data_model::revision::FieldRevision;
use strum::EnumCount;

pub struct GridRowTestBuilder<'a> {
    test: &'a GridEditorTest,
    inner_builder: CreateRowRevisionBuilder<'a>,
}

impl<'a> GridRowTestBuilder<'a> {
    pub fn new(test: &'a GridEditorTest) -> Self {
        assert_eq!(test.field_revs.len(), FieldType::COUNT);

        let inner_builder = CreateRowRevisionBuilder::new(&test.field_revs);
        Self { test, inner_builder }
    }

    pub fn update_text_cell(&mut self) -> Self {
        let text_field = self
            .test
            .field_revs
            .iter()
            .find(|field_rev| field_rev.field_type == FieldType::RichText);
        // self.inner_builder
    }

    pub fn build(self) -> CreateRowRevisionPayload {
        self.inner_builder.build()
    }
}
