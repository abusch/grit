use crate::screen::CommitInfo;

pub struct Graph {
    row: GraphRow,
    parents: GraphRow,
    prev_row: GraphRow,
    next_row: GraphRow,
    position: usize,
    prev_position: usize,
    expanded: usize,
    id: String,
    has_parents: bool,
    is_boundary: bool,
}

impl Graph {
    pub fn add_commit(&mut self, commit: &CommitInfo) {}
}

pub struct GraphSymbol {
    // unsigned int color:8;

    // TODO use a bitset instead?
    commit: bool,
    boundary: bool,
    initial: bool,
    merge: bool,

    continued_down: bool,
    continued_up: bool,
    continued_right: bool,
    continued_left: bool,
    continued_up_left: bool,

    parent_down: bool,
    parent_right: bool,

    below_commit: bool,
    flanked: bool,
    next_right: bool,
    matches_commit: bool,

    shift_left: bool,
    continue_shift: bool,
    below_shift: bool,

    new_column: bool,
    empty: bool,
}

pub struct GraphColumn {
    symbol: GraphSymbol,
    /// Parent SHA1 id
    id: String,
}

struct GraphRow {
    columns: Vec<GraphColumn>,
}

impl GraphRow {}
