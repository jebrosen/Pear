#[macro_export]
macro_rules! is_debug {
    ($($e:tt)*) => ({
        #[cfg(debug_assertions)]
        let result = ::std::env::var("PEAR_DEBUG").is_ok();
        #[cfg(not(debug_assertions))]
        let result = false;
        result
    })
}

#[macro_export]
macro_rules! declare {
    ($input:ident $(<$($gen:tt),+>)* (Token = $t:ty, Slice = $s:ty, Many = $m:ty)) => {
        declare!($input $(<$($gen),+>)*($t, $s, $s, $m));
    };

    ($input:ident $(<$($gen:tt),+>)* (Token = $t:ty, Slice = $s:ty, InSlice = $is:ty, Many = $m:ty)) => {
        declare!($input $(<$($gen),+>)*($t, $s, $is, $m));
    };

    ($input:ident $(<$($gen:tt),+>)* ($t:ty, $s:ty, $is:ty, $m:ty)) => {
        trait $input $(<$($gen),+>)*: $crate::Input<Token=$t, Slice=$s, InSlice=$is, Many=$m> {  }

        impl<$($($gen,)+)* T> $input $(<$($gen)+>)* for T
            where T: $crate::Input<Token=$t, Slice=$s, InSlice=$is, Many=$m> + $($($gen),+)* {  }
    }
}

#[macro_export]
macro_rules! parse {
    ($parser:ident : $e:expr) => ({
        let input = $e;
        (move || {
            let result = $parser(input)?;
            $crate::parsers::eof(input)?;
            $crate::AsResult::as_result(result)
        })()
    })
}

#[macro_export]
macro_rules! pear_error {
    ([$name:ident; $i:expr] $err:expr) => (pear_error!([$name; $i] $err,));
    ([$name:ident; $i:expr] $fmt:expr, $($arg:tt)*) => {
        $crate::ParseErr::from_context($i, stringify!($name), format!($fmt, $($arg)*))
    };
}

/// FIXME: This is an issue with rustc here where if `$input` is `expr`
/// everything fails.
#[macro_export]
macro_rules! pear_try {
    ([$name:ident; $input:ident] $e:expr) => {{
        switch! { [$name;$input] result@$e => { Some(result) }, _ => { None } }
    }};
    ([$name:ident; $input:ident] $e:expr => $r:expr) => {{
        switch! { [$name;$input] $e => { Some($r) }, _ => { None } }
    }};
    ([$name:ident; $input:ident] $pat:ident@$e:expr => $r:expr) => {{
        switch! { [$name;$input] $pat@$e => { Some($r) }, _ => { None } }
    }}
}
