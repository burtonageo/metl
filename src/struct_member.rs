use cocoa::base::id;

struct StructMember(id);

impl_from_into_raw!(StructMember, of class "MTLStructMember");
