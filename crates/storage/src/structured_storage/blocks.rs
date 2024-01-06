//! The module contains implementations and tests for the `FuelBlocks` table.

use crate::{
    codec::{
        postcard::Postcard,
        primitive::Primitive,
    },
    column::Column,
    structure::plain::Plain,
    structured_storage::TableWithStructure,
    tables::FuelBlocks,
};

impl TableWithStructure for FuelBlocks {
    type Structure = Plain<Primitive<4>, Postcard>;

    fn column() -> Column {
        Column::FuelBlocks
    }
}

#[cfg(test)]
crate::basic_storage_tests!(
    FuelBlocks,
    <FuelBlocks as crate::Mappable>::Key::default(),
    <FuelBlocks as crate::Mappable>::Value::default()
);
