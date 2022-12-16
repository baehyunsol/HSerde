use std::collections::HashMap;

pub enum HSObj {
    Int(i32 /* TODO: inf precision integer */),
    Real(f64),
    Table(HashMap<HSObj, HSObj>),
    Vec(Vec<Box<HSObj>>),
    True, False, None,
    String(String)
}