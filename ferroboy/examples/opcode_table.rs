use ferroboy::{Disassemble, Operation};
use prettytable::{Cell, Row, Table};

fn print_header(table: &mut Table) {
    let mut cells: Vec<Cell> = Vec::new();
    cells.push(Cell::new(""));

    for value in 0x0..=0xF {
        let text = format!("{:02X}", value);
        cells.push(Cell::new(&text));
    }

    table.add_row(Row::new(cells));
}

fn print_row(table: &mut Table, leading: u8, opcodes: &[Option<&dyn Operation>]) {
    let mut cells: Vec<Cell> = Vec::new();
    let head = format!("{:02X}", leading);
    cells.push(Cell::new(&head));

    for operation in opcodes {
        if let Some(op) = operation {
            let description = format!("{}", op.describe().unwrap());
            cells.push(Cell::new(&description))
        } else {
            cells.push(Cell::new("-"))
        }
    }

    table.add_row(Row::new(cells));
}

fn main() {
    let mut table = Table::new();

    print_header(&mut table);

    for first_half in 0x0..=0xF {
        let leader = first_half * 0x10;
        let mut ops: Vec<Option<&dyn Operation>> = Vec::new();

        for second_half in 0x0..=0xF {
            let opcode = leader + second_half;
            let operation = ferroboy::OPCODES.get(&opcode).copied();
            ops.push(operation);
        }

        print_row(&mut table, leader, &ops);
    }

    table.printstd();
}
