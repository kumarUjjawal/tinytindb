# SQLite Clone in Rust

## 0. Current State (What We Already Have)

### Modules

- **input_buffer.rs** – reads user input, prints prompt
- **row.rs** – defines Row + username/email sizes
- **statement.rs** – StatementType, Statement, MetaCommandResult, PrepareResult
- **parser.rs** – parses meta commands + insert/select
- **table.rs** – Table, paging constants, row_slot, serialize/deserialize
- **executor.rs** – executes INSERT and SELECT using Table
- **main.rs** – REPL loop, wires everything together

### Current Features

- [x] Structured REPL
- [x] `insert <id> <username> <email>`
- [x] `select` printing all rows
- [x] In-memory pages with row serialization

---

##  1 – Stabilize Current Design

### 1.1 input_buffer module

- [ ] Add small unit tests for:
  - `read_input` trimming behavior
  - Handling empty lines (just Enter)
- [ ] Add a helper like:
  ```rust
  pub fn is_meta_command(&self) -> bool
  ```
  so we don't check `starts_with('.')` everywhere manually.

### 1.2 row module

- [ ] Add functions for nicer conversions:
  - `Row::from_values(id: u32, username: &str, email: &str) -> Result<Row, Error>`
  - `Row::username_as_str(&self) -> &str` (trim trailing zeros)
  - `Row::email_as_str(&self) -> &str`
- [ ] Add basic tests:
  - Username/email truncation or error when too long
  - Round-trip through `Row::from_values` and the accessors

### 1.3 statement module

- [ ] Add more specific error variants if needed later:
  - e.g. `PrepareResult::StringTooLong`, `NegativeId`, etc. (optional for now)
- [ ] Implement `Display` for `StatementType` and/or `Statement` for debug prints.

### 1.4 table module

- [ ] Add tests for layout constants:
  - `ROW_SIZE * ROWS_PER_PAGE <= PAGE_SIZE`
  - `TABLE_MAX_ROWS == ROWS_PER_PAGE * TABLE_MAX_PAGES`
- [ ] Add test verifying:
  - `row_slot` for `row_num = 0`, `ROWS_PER_PAGE - 1`, `ROWS_PER_PAGE`, etc. hits correct offsets.
- [ ] Separate "layout" & "serialization" into submodules/files later:
  - `table/layout.rs` – constants, offsets
  - `table/serialization.rs` – serialize/deserialize
  - `table/mod.rs` – Table + row_slot

### 1.5 parser module

- [ ] Extract parsing logic into helper functions:
  - `parse_insert(input: &str) -> Result<Row, PrepareResult>`
  - `is_insert_command(cmd: &str)` (optional)
- [ ] Add tests:
  - Correct parsing: `insert 1 alice alice@example.com`
  - Missing args → `SyntaxError`
  - Non-numeric id → `SyntaxError`
  - Too-long username/email → `SyntaxError` (or specific error variant)

### 1.6 executor module

- [ ] Separate execution functions:
  - `execute_insert(statement, table)`
  - `execute_select(statement, table)`
- [ ] Add tests:
  - Insert one row, select prints it (or returns rows in a test-friendly way)
  - Insert multiple rows, select returns all in order

### 1.7 main module

- [ ] Clean up the REPL:
  - Factor meta-command handling into a function:
    ```rust
    fn handle_meta_command(input: &InputBuffer) -> bool
    ```
    returning "handled or not".
  - Pretty error messages for `SyntaxError` and `UnrecognizedStatement`.

---

##  2 – Add Persistence: Pager + Database File

**Goal:** Move from in-memory only to file-backed pages.

### 2.1 Create pager.rs

- [ ] Define a `Pager` struct:
  ```rust
  pub struct Pager {
      file: std::fs::File,
      file_length: u64,
      pages: [Option<Box<[u8; PAGE_SIZE]>>; TABLE_MAX_PAGES],
  }
  ```
- [ ] Implement:
  - `Pager::open(filename: &str) -> Result<Pager, Error>`
  - `Pager::get_page(&mut self, page_num: usize) -> &mut [u8; PAGE_SIZE]`
    - Lazily loads from disk (seek + read) or allocates new page
  - `Pager::flush_page(&mut self, page_num: usize, num_pages: usize)` (or `flush_all`)
- [ ] Handle file size:
  - Compute number of pages based on `file_length`
  - Later we'll use this to compute `num_rows` on open.

### 2.2 Update table.rs to use Pager

- [ ] Change `Table` structure:
  ```rust
  pub struct Table {
      pub num_rows: usize,
      pub pager: Pager,
  }
  ```
- [ ] Replace pages array usage with calls to `pager.get_page(page_num)`.
- [ ] Decide:
  - Where to store `num_rows` (in memory or in file header initially).
  - Early version can keep `num_rows` in memory only and recompute on startup as `file_length / ROW_SIZE`.
- [ ] Add:
  - `Table::open(filename: &str) -> Result<Table, Error>`
  - `impl Drop for Table` to flush pages on close.

### 2.3 main.rs: open DB file

- [ ] Start program with:
  - Accept filename from CLI: `cargo run test.db`
  - `let mut table = Table::open(filename)?;`
- [ ] Ensure:
  - On program exit (or `.exit`), table flushes all pages to disk.

---

##  3 – Introduce Cursor Abstraction

**Goal:** Avoid calculating row offsets everywhere and prepare for B-tree.

### 3.1 cursor.rs

- [ ] Create new module:
  ```rust
  pub struct Cursor {
      pub table: *mut Table,  // or &mut Table with lifetimes
      pub row_num: usize,
      pub end_of_table: bool,
  }
  ```
- [ ] Implement:
  - `Cursor::table_start(table: &mut Table) -> Cursor`
  - `Cursor::table_end(table: &mut Table) -> Cursor` (for insert at end)
  - `cursor_value(&mut self) -> &mut [u8]` (wraps `row_slot`)
  - `cursor_advance(&mut self)` (increments `row_num`, sets `end_of_table` when done)
- [ ] Note: Later, cursor will hold `page_num` and `cell_num` for B-tree, but for now it's just row index.

### 3.2 Update executor.rs to use Cursor

- [ ] In `execute_insert`:
  - Use `Cursor::table_end(&mut table)` to find insert position.
  - Use `cursor_value()` to get `&mut [u8]` and `serialize_row`.
- [ ] In `execute_select`:
  - Use `Cursor::table_start(&mut table)`
  - Loop: `while not end_of_table`:
    - `cursor_value()` → `deserialize_row`
    - `cursor_advance()`
- [ ] This isolates the logic of "how to walk rows" into the cursor.

---

##  4 – B-Tree Node Layout (Still Single Leaf Page)

**Goal:** Switch from flat rows to B-tree shaped layout, starting with leaf nodes only.

### 4.1 btree_layout.rs or node.rs

- [ ] Define node constants:
  - `NODE_TYPE_OFFSET`, `IS_ROOT_OFFSET`, `PARENT_POINTER_OFFSET`, etc.
  - Leaf node header: `LEAF_NODE_NUM_CELLS_OFFSET`, `LEAF_NODE_HEADER_SIZE`
  - Leaf body: `LEAF_NODE_CELL_SIZE`, `LEAF_NODE_KEY_SIZE`, `LEAF_NODE_VALUE_SIZE`
  - Max cells: `LEAF_NODE_MAX_CELLS`
- [ ] Functions for working with leaf nodes:
  - `leaf_node_num_cells(node: &mut [u8]) -> &mut u32`
  - `leaf_node_cell(node: &mut [u8], cell_num: usize) -> &mut [u8]`
  - `leaf_node_key(cell: &[u8]) -> &u32`
  - `leaf_node_value(cell: &mut [u8]) -> &mut [u8]` (Row bytes)
- [ ] Initialize new root as a leaf node.

### 4.2 Adapt Table + Cursor to B-tree leaf nodes

- [ ] Make `Table` have:
  - `root_page_num: usize`
- [ ] `Cursor` now stores:
  ```rust
  pub struct Cursor {
      pub table: *mut Table,
      pub page_num: usize,
      pub cell_num: usize,
      pub end_of_table: bool,
  }
  ```
- [ ] Implement functions:
  - `Table::leaf_node_start(&mut self)`
  - `Cursor::value()` returning row's value slice inside leaf node.

### 4.3 Insert into leaf node without splits

- [ ] Implement `leaf_node_insert()` that:
  - Shifts existing cells to make room.
  - Writes key (id) and serialized row.
- [ ] For now, assume no splits:
  - If leaf is full → error or "Need to implement leaf node split".

---

##  5 – Leaf Node Search and Splits

### 5.1 Binary search on leaf node

- [ ] Implement `leaf_node_find(node, key) -> Cursor`:
  - Binary search on keys
  - Returns cursor where key is or should be inserted.
- [ ] Update insert to:
  - Use `leaf_node_find` instead of always inserting at end.

### 5.2 Splitting a leaf node

- [ ] Implement:
  - `leaf_node_split_and_insert(...)` that:
    - Creates a new leaf page.
    - Redistributes cells between old and new.
    - Updates parent/root pointers accordingly.
- [ ] Initially:
  - Support only splitting the root (single-level tree).

---

##  6 – Internal Nodes and Multi-Level B-Tree

### 6.1 Internal node layout

- [ ] Define internal-node constants:
  - `INTERNAL_NODE_NUM_KEYS`, `INTERNAL_NODE_CHILD_OFFSET`, etc.
- [ ] Implement utilities:
  - `internal_node_num_keys(node: &mut [u8]) -> &mut u32`
  - `internal_node_key(node, key_index) -> &u32`
  - `internal_node_child(node, child_index) -> &u32`

### 6.2 Searching through internal nodes

- [ ] Implement:
  - `internal_node_find_child(node, key)`
  - Recursive `table_find(table, key)` that:
    - Traverses internal nodes
    - Ends at leaf node
- [ ] Update `Cursor::table_start` / search operations to use this logic.

### 6.3 Splitting internal nodes

- [ ] Implement:
  - `create_new_root(...)` that:
    - Makes old root a child.
    - Creates a new internal root with two children.
- [ ] Implement:
  - Splitting internal node when full.

---

##  7 – Meta Commands & Debugging

### 7.1 .constants and .btree

- [ ] Extend `do_meta_command` to support:
  - `.constants` – prints layout constants (`PAGE_SIZE`, `ROW_SIZE`, etc.)
  - `.btree` – prints a textual representation of the tree:
    - Node type, keys, child pointers.
- [ ] Implement pretty-printer for tree in a new module `debug.rs`.

---

##  8 – Quality of Life & Extras

- [ ] Add a simple testing harness:
  - Run script of commands and check output (like the tutorial does with Python).
- [ ] Add:
  - `.help` to show available commands.
- [ ] Possibly:
  - Support `delete` (harder, but optional)
  - Basic `WHERE` clauses (e.g. `select where id = X` – trivial filter layer on top of scan or search)

---

