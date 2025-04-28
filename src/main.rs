mod list;

use list::List;

fn main() {
    let mut l = List::new();

    l.append(5);
    l.append(10);
    l.append(25);
    l.prepend(0);

    l.print_values(" | ");
}