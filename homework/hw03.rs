const WIDTH: usize = 30;
const HEIGHT: usize = 14;

pub fn draw_envelope() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c = if y == 0 || y == HEIGHT - 1 {
                '*'
            } else if x == 0 || x == WIDTH - 1 {
                '*'
            } else if x == y || x == WIDTH - 1 - y {
                '*'
            } else {
                ' '
            };
            output.push(c);
        }
        output.push('\n');
    }

    print!("{}", output);
}
