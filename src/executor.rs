use crate::{
    row::Row,
    statement::{Statement, StatementType},
    table::{Table, deserialize_row, serialize_row},
};

pub fn execute_statement(statement: &Statement, table: &mut Table) {
    match statement.stype {
        StatementType::Insert => {
            let row = statement.row_to_insert.as_ref().unwrap();
            let slot = table.row_slot(table.num_rows);

            serialize_row(row, slot);
            table.num_rows += 1;
        }
        StatementType::Select => {
            for i in 0..table.num_rows {
                let page_row = table.row_slot(i).to_vec();
                let mut row = Row::empty();
                deserialize_row(&page_row, &mut row);
                println!(
                    "{}, {}, {}",
                    row.id,
                    String::from_utf8_lossy(&row.username),
                    String::from_utf8_lossy(&row.email)
                );
            }
        }
    }
}
