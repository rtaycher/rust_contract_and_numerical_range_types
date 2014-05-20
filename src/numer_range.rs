#[macro_escape]
pub mod numer_range {

    use std::to_str::ToStr;

    #[macro_export]
    macro_rules! make_range_int_type(
        ($RangedIntName:ident $RIN:ident $OORN:ident $l:expr $u:expr) => (

            #[deriving(Show)]
            enum $RangedIntName {
                $OORN,
                $RIN(int)
            }

            impl Add<$RangedIntName, $RangedIntName> for $RangedIntName {
                fn add(&self, _rhs: &$RangedIntName) -> $RangedIntName {
                    match (*self, *_rhs) {
                               ($RIN(x),$RIN(y)) => {
                                   let value = x + y;
                                   if $RangedIntName::is_in_range(value) { $RIN(value) } else { $OORN }
                               },
                               ($OORN, _) => $OORN,
                               (_, $OORN) => $OORN
                    }
                }
            }

            impl $RangedIntName {
                 fn from_int(i: int) -> $RangedIntName {
                    if $RangedIntName::is_in_range(i) { $RIN(i) } else { $OORN }
                }

                fn is_in_range(i: int) -> bool {
                    i>=$l && i <= $u
                }

                fn unwrap(self) -> int {
                        match self {
                            $RIN(x) => x,
                            $OORN => fail!("failed to unwrap $RangedIntName - OutOfRange!")
                        }
                }
            }

        );
    )

}