use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::type_dart_generator_struct;
use crate::utils::BlockIndex;

type_dart_generator_struct!(TypeGeneralListGenerator, IrTypeGeneralList);

impl TypeDartGeneratorTrait for TypeGeneralListGenerator<'_> {
    fn api2wire_body(&self, block_index: BlockIndex) -> Option<String> {
        // NOTE the memory strategy is same as PrimitiveList, see comments there.
        Some(format!(
            "final ans = inner.new_{}_{}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    _api_fill_to_wire_{}(raw[i], ans.ref.ptr[i]);
                }}
                return ans;",
            self.ir.safe_ident(),
            block_index,
            self.ir.inner.safe_ident()
        ))
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return (raw as List<dynamic>).map(_wire2api_{}).toList();",
            self.ir.inner.safe_ident()
        )
    }
}
