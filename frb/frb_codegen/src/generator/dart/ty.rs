use crate::generator::dart::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait TypeDartGeneratorTrait {
    fn api2wire_body(&self, block_index: BlockIndex) -> Option<String>;

    fn api_fill_to_wire_body(&self) -> Option<String> {
        None
    }

    fn wire2api_body(&self) -> String {
        "".to_string()
    }

    fn structs(&self) -> String {
        "".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct TypeGeneratorContext<'a> {
    pub ir_file: &'a IrFile,
}

#[macro_export]
macro_rules! type_dart_generator_struct {
    ($cls:ident, $ir_cls:ty) => {
        #[derive(Debug, Clone)]
        pub struct $cls<'a> {
            pub ir: $ir_cls,
            pub context: TypeGeneratorContext<'a>,
            pub dart_api_class_name: Option<String>,
        }
    };
}

#[enum_dispatch(TypeDartGeneratorTrait)]
#[derive(Debug, Clone)]
pub enum TypeDartGenerator<'a> {
    Primitive(TypePrimitiveGenerator<'a>),
    Delegate(TypeDelegateGenerator<'a>),
    PrimitiveList(TypePrimitiveListGenerator<'a>),
    Optional(TypeOptionalGenerator<'a>),
    GeneralList(TypeGeneralListGenerator<'a>),
    StructRef(TypeStructRefGenerator<'a>),
    Boxed(TypeBoxedGenerator<'a>),
    EnumRef(TypeEnumRefGenerator<'a>),
}

impl<'a> TypeDartGenerator<'a> {
    pub fn new(ty: IrType, ir_file: &'a IrFile, dart_api_class_name: Option<String>) -> Self {
        let context = TypeGeneratorContext { ir_file };
        match ty {
            Primitive(ir) => TypePrimitiveGenerator {
                ir,
                context,
                dart_api_class_name,
            }
            .into(),
            Delegate(ir) => TypeDelegateGenerator {
                ir,
                context,
                dart_api_class_name,
            }
            .into(),
            PrimitiveList(ir) => TypePrimitiveListGenerator {
                ir,
                context,
                dart_api_class_name,
            }
            .into(),
            Optional(ir) => TypeOptionalGenerator {
                ir,
                context,
                dart_api_class_name,
            }
            .into(),
            GeneralList(ir) => TypeGeneralListGenerator {
                ir,
                context,
                dart_api_class_name,
            }
            .into(),
            StructRef(ir) => TypeStructRefGenerator {
                ir,
                context,
                dart_api_class_name,
            }
            .into(),
            Boxed(ir) => TypeBoxedGenerator {
                ir,
                context,
                dart_api_class_name,
            }
            .into(),
            EnumRef(ir) => TypeEnumRefGenerator {
                ir,
                context,
                dart_api_class_name,
            }
            .into(),
        }
    }
}
