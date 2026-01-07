#![cfg(feature = "dashu-int")]

use dashu_int::{IBig, UBig};

use crate::{
    Def, Facet, PtrConst, Shape, ShapeBuilder, TryFromOutcome, Type, UserType, VTableDirect,
    vtable_direct,
};

unsafe fn ibig_try_from(
    dst: *mut IBig,
    src_shape: &'static Shape,
    src: PtrConst,
) -> TryFromOutcome {
    unsafe {
        // Handle &str
        if src_shape.id == <&str as Facet>::SHAPE.id {
            let source_str = src.get::<&str>();
            match source_str.parse::<IBig>() {
                Ok(val) => {
                    dst.write(val);
                    TryFromOutcome::Converted
                }
                Err(e) => TryFromOutcome::Failed(format!("IBig parsing failed: {e}").into()),
            }
        }
        // Handle String
        else if src_shape.id == <String as Facet>::SHAPE.id {
            let source_str = src.read::<String>();
            match source_str.parse::<IBig>() {
                Ok(val) => {
                    dst.write(val);
                    TryFromOutcome::Converted
                }
                Err(e) => TryFromOutcome::Failed(format!("IBig parsing failed: {e}").into()),
            }
        }
        // Handle i64
        else if src_shape.id == <i64 as Facet>::SHAPE.id {
            let source_int = src.read::<i64>();
            let val = IBig::from(source_int);
            dst.write(val);
            TryFromOutcome::Converted
        }
        // Handle u64
        else if src_shape.id == <u64 as Facet>::SHAPE.id {
            let source_int = src.read::<u64>();
            let val = IBig::from(source_int);
            dst.write(val);
            TryFromOutcome::Converted
        }
        // Unsupported source shape
        else {
            TryFromOutcome::Unsupported
        }
    }
}

unsafe impl<'facet> Facet<'facet> for IBig {
    const SHAPE: &'static Shape = &const {
        const VTABLE: VTableDirect = vtable_direct!(IBig =>
            Display,
            Debug,
            Hash,
            PartialEq,
            PartialOrd,
            Ord,
            FromStr,
            [try_from = ibig_try_from],
        );

        ShapeBuilder::for_sized::<Self>("IBig")
            .ty(Type::User(UserType::Opaque))
            .def(Def::Scalar)
            .vtable_direct(&VTABLE)
            .eq()
            .send()
            .sync()
            .build()
    };
}

unsafe fn ubig_try_from(
    dst: *mut UBig,
    src_shape: &'static Shape,
    src: PtrConst,
) -> TryFromOutcome {
    unsafe {
        // Handle &str
        if src_shape.id == <&str as Facet>::SHAPE.id {
            let source_str = src.get::<&str>();
            match source_str.parse::<UBig>() {
                Ok(val) => {
                    dst.write(val);
                    TryFromOutcome::Converted
                }
                Err(e) => TryFromOutcome::Failed(format!("UBig parsing failed: {e}").into()),
            }
        }
        // Handle String
        else if src_shape.id == <String as Facet>::SHAPE.id {
            let source_str = src.read::<String>();
            match source_str.parse::<UBig>() {
                Ok(val) => {
                    dst.write(val);
                    TryFromOutcome::Converted
                }
                Err(e) => TryFromOutcome::Failed(format!("UBig parsing failed: {e}").into()),
            }
        }
        // Handle u64
        else if src_shape.id == <u64 as Facet>::SHAPE.id {
            let source_int = *src.get::<u64>();
            let val = UBig::from(source_int);
            dst.write(val);
            TryFromOutcome::Converted
        }
        // Unsupported source shape
        else {
            TryFromOutcome::Unsupported
        }
    }
}

unsafe impl<'facet> Facet<'facet> for UBig {
    const SHAPE: &'static Shape = &const {
        const VTABLE: VTableDirect = vtable_direct!(UBig =>
            Display,
            Debug,
            Hash,
            PartialEq,
            PartialOrd,
            Ord,
            FromStr,
            [try_from = ubig_try_from],
        );

        ShapeBuilder::for_sized::<Self>("UBig")
            .ty(Type::User(UserType::Opaque))
            .def(Def::Scalar)
            .vtable_direct(&VTABLE)
            .eq()
            .send()
            .sync()
            .build()
    };
}

#[cfg(test)]
mod tests {
    use dashu_int::{IBig, UBig};

    use crate::{Facet, Shape, ShapeLayout};

    #[test]
    fn test_ibig() {
        const SHAPE: &Shape = IBig::SHAPE;
        assert_eq!(SHAPE.type_identifier, "IBig");
        assert!(matches!(SHAPE.layout, ShapeLayout::Sized(..)));
        let layout = SHAPE.layout.sized_layout().unwrap();
        assert_eq!(layout.size(), 24);
        assert_eq!(layout.align(), 8);
    }

    #[test]
    fn test_ubig() {
        const SHAPE: &Shape = UBig::SHAPE;
        assert_eq!(SHAPE.type_identifier, "UBig");
        assert!(matches!(SHAPE.layout, ShapeLayout::Sized(..)));
        let layout = SHAPE.layout.sized_layout().unwrap();
        assert_eq!(layout.size(), 24);
        assert_eq!(layout.align(), 8);
    }
}
