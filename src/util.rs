#[macro_export]
macro_rules! return_if {
    ($cond:expr, $ret:expr) => {
        if $cond {
            return $ret;
        }
    };
}

#[macro_export]
macro_rules! return_if_else {
    ($cond:expr, $ret:expr, $else:expr) => {
        if $cond {
            return $ret;
        } else {
            return $else;
        }
    };
}

#[macro_export]
macro_rules! break_if {
    ($cond:expr) => {
        if $cond {
            break;
        }
    };
}

pub fn chess_to_checkers_id(id: &u8) -> u8 {
    let row = *id / 8;
    let col = *id % 8;
    row * 4 + col / 2
}
