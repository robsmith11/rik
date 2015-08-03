// For now just the KObject enum. will including parsing code
// as it gets extracted from the protocol code

#[derive(Debug)]
pub enum KObject {
    // Atoms
    BooleanAtom   (u8        ),
    GuidAtom      ((u64, u64)),
    ByteAtom      (i8        ),
    ShortAtom     (i16       ),
    IntAtom       (i32       ),
    LongAtom      (i64       ),
    RealAtom      (f32       ),
    FloatAtom     (f64       ),
    CharAtom      (u8        ),
    SymbolAtom    (String    ),
    TimestampAtom (i64       ),
    MonthAtom     (i32       ),
    DateAtom      (i32       ),
    DateTimeAtom  (f64       ),
    TimespanAtom  (i64       ),
    MinuteAtom    (i32       ),
    SecondAtom    (i32       ),
    TimeAtom      (i32       ),

    // Vectors
    GeneralList     (Vec<KObject>   ),
    BooleanVector   (Vec<u8>        ),
    GuidVector      (Vec<(u64, u64)>),
    ByteVector      (Vec<i8>        ),
    ShortVector     (Vec<i16>       ),
    IntVector       (Vec<i32>       ),
    LongVector      (Vec<i64>       ),
    RealVector      (Vec<f32>       ),
    FloatVector     (Vec<f64>       ),
    CharVector      (Vec<u8>        ),
    SymbolVector    (Vec<String>    ),
    TimestampVector (Vec<i64>       ),
    MonthVector     (Vec<i32>       ),
    DateVector      (Vec<i32>       ),
    DateTimeVector  (Vec<f64>       ),
    TimespanVector  (Vec<i64>       ),
    MinuteVector    (Vec<i32>       ),
    SecondVector    (Vec<i32>       ),
    TimeVector      (Vec<i32>       ),

    // Composites
    // TODO: Strucutre into subtyps so I can say (Vector, Vector)
    Dict              ((KObject, KObject)),
    SortedDict        ((Kobject, KObject)),
    Table             ((KObject, KObject)),

    // TODO: These are really (Table, Table)
    KeyedTable        ((KObject, KObject)),
    SortedeKeyedTable ((KObject, KObject)),

    // Other
    // TODO: maybe parse functions propertly?
    Function   (Vec<u8>),
    UnknownObj (Vec<u8>),
}

