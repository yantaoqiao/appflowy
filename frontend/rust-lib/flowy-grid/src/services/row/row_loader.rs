use crate::services::row::decode_cell_data;
use flowy_error::FlowyResult;
use flowy_grid_data_model::entities::{
    Cell, CellMeta, FieldMeta, GridBlock, GridBlockOrder, RepeatedGridBlock, Row, RowMeta, RowOrder,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct GridBlockSnapshot {
    pub(crate) block_id: String,
    pub row_metas: Vec<Arc<RowMeta>>,
}

pub(crate) fn group_row_orders(row_orders: Vec<RowOrder>) -> Vec<GridBlockOrder> {
    let mut map: HashMap<String, GridBlockOrder> = HashMap::new();
    row_orders.into_iter().for_each(|row_order| {
        // Memory Optimization: escape clone block_id
        let block_id = row_order.block_id.clone();
        map.entry(block_id)
            .or_insert_with(|| GridBlockOrder::new(&row_order.block_id))
            .row_orders
            .push(row_order);
    });
    map.into_values().collect::<Vec<_>>()
}

#[inline(always)]
pub fn make_cell_by_field_id(
    field_map: &HashMap<&String, &FieldMeta>,
    field_id: String,
    cell_meta: CellMeta,
) -> Option<(String, Cell)> {
    let field_meta = field_map.get(&field_id)?;
    let content = decode_cell_data(cell_meta.data, field_meta, &field_meta.field_type)?;
    let cell = Cell::new(&field_id, content);
    Some((field_id, cell))
}

#[allow(dead_code)]
pub fn make_cell(field_id: &str, field_meta: &FieldMeta, row_meta: &RowMeta) -> Option<Cell> {
    let cell_meta = row_meta.cells.get(field_id)?.clone();
    let content = decode_cell_data(cell_meta.data, field_meta, &field_meta.field_type)?;
    Some(Cell::new(field_id, content))
}

pub(crate) fn make_row_orders_from_row_metas(row_metas: &[Arc<RowMeta>]) -> Vec<RowOrder> {
    row_metas.iter().map(RowOrder::from).collect::<Vec<_>>()
}

pub(crate) fn make_row_from_row_meta(fields: &[FieldMeta], row_meta: Arc<RowMeta>) -> Option<Row> {
    make_rows_from_row_metas(fields, &[row_meta]).pop()
}

pub(crate) fn make_rows_from_row_metas(fields: &[FieldMeta], row_metas: &[Arc<RowMeta>]) -> Vec<Row> {
    let field_meta_map = fields
        .iter()
        .map(|field_meta| (&field_meta.id, field_meta))
        .collect::<HashMap<&String, &FieldMeta>>();

    let make_row = |row_meta: &Arc<RowMeta>| {
        let cell_by_field_id = row_meta
            .cells
            .clone()
            .into_iter()
            .flat_map(|(field_id, cell_meta)| make_cell_by_field_id(&field_meta_map, field_id, cell_meta))
            .collect::<HashMap<String, Cell>>();

        Row {
            id: row_meta.id.clone(),
            cell_by_field_id,
            height: row_meta.height,
        }
    };

    row_metas.iter().map(make_row).collect::<Vec<_>>()
}

pub(crate) fn make_grid_blocks(
    block_ids: Option<Vec<String>>,
    block_snapshots: Vec<GridBlockSnapshot>,
) -> FlowyResult<RepeatedGridBlock> {
    match block_ids {
        None => Ok(block_snapshots
            .into_iter()
            .map(|snapshot| {
                let row_orders = make_row_orders_from_row_metas(&snapshot.row_metas);
                GridBlock::new(&snapshot.block_id, row_orders)
            })
            .collect::<Vec<GridBlock>>()
            .into()),
        Some(block_ids) => {
            let block_meta_data_map: HashMap<&String, &Vec<Arc<RowMeta>>> = block_snapshots
                .iter()
                .map(|data| (&data.block_id, &data.row_metas))
                .collect();

            let mut grid_blocks = vec![];
            for block_id in block_ids {
                match block_meta_data_map.get(&block_id) {
                    None => {}
                    Some(row_metas) => {
                        let row_orders = make_row_orders_from_row_metas(row_metas);
                        grid_blocks.push(GridBlock::new(&block_id, row_orders));
                    }
                }
            }
            Ok(grid_blocks.into())
        }
    }
}
