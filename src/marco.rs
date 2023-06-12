#[macro_export]
macro_rules! async_writeln {
    ($dst: expr) => {
        {
            tokio::io::AsyncWriteExt::write_all(&mut $dst, b"\n").await
        }
    };
    ($dst: expr, $fmt: expr) => {
        {
            use std::io::Write;
            let mut buf = Vec::<u8>::new();
            writeln!(buf, $fmt).unwrap();
            tokio::io::AsyncWriteExt::write_all(&mut $dst, &buf).await
        }
    };
    ($dst: expr, $fmt: expr, $($arg: tt)*) => {
        {
            use std::io::Write;
            let mut buf = Vec::<u8>::new();
            writeln!(buf, $fmt, $( $arg )*).unwrap();
            tokio::io::AsyncWriteExt::write_all(&mut $dst, &buf).await
        }
    };
}
