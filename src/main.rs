thread_local! {
    static STDOUT: std::cell::RefCell<std::io::BufWriter<std::io::StdoutLock<'static>>> = std::cell::RefCell::new(std::io::BufWriter::with_capacity(1 << 17, std::io::stdout().lock()));
}
 
#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {
        STDOUT.with(|refcell| {
            use std::io::*;
            writeln!(refcell.borrow_mut(), $($t)*).unwrap();
        });
    };
}
#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {
        STDOUT.with(|refcell| {
            use std::io::*;
            write!(refcell.borrow_mut(), $($t)*).unwrap();
        });
    };
}

macro_rules! read_line {
    ( $i: expr ) => {
        $i.next().unwrap()
    };
}

macro_rules! read {
    ( $i: expr, $t: ty ) => {
        read_line!($i).parse::<$t>().unwrap()
    };
}

macro_rules! reads {
    ( $i: expr, $t: ty ) => {
        read_line!($i).split_whitespace().map(|i| i.parse::<$t>().unwrap()).collect()
    };
    ( $i: expr, $x:ty, $( $y: ty ), *) => {
        {
            let mut data = read_line!($i).split_whitespace();
            (data.next().unwrap().parse::<$x>().unwrap(), $(
                data.next().unwrap().parse::<$y>().unwrap(), 
            )*)
        }
    };
}

fn solve(_stdin: &str) {
    let mut lines = _stdin.split('\n');
}
 
fn main() {
    use std::io::*;
    solve(&read_to_string(stdin()).unwrap());
    STDOUT.with(|refcell| std::io::Write::flush(&mut *refcell.borrow_mut()).unwrap());
}
 
