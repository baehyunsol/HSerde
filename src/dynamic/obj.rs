use std::collections::HashMap;

pub enum HSObj {
    Int(i32 /* TODO: inf precision integer */),
    Real(f64),
    Table(HashMap<HSObj, HSObj>),
    Vec(Vec<Box<HSObj>>),
    True, False, None,
    String(String)
}

impl HSObj {

    // TODO: (opt suggestion) get `&mut Vec<u8>` as arg and append the result to its end
    pub fn serial_text(&self) -> Vec<u8> {

        match self {
            HSObj::Int(n) => todo!(),
            HSObj::Real(n) => todo!(),
            HSObj::Table(t) => todo!(),
            HSObj::Table(t) => todo!(),
            HSObj::Vec(v) => todo!(),
            HSObj::True => todo!(),
            HSObj::False => todo!(),
            HSObj::None => todo!(),
            HSObj::String(s) => todo!(),
        }

    }

    pub fn serial_binary(&self) -> Vec<u8> {
        todo!()
    }

}