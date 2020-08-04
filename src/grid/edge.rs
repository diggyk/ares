use diesel::backend::Backend;
use diesel::deserialize::{FromSql, Result};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use serde::Serialize;
use std::io::Write;

#[repr(i16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, AsExpression, FromSqlRow, Serialize)]
#[sql_type = "SmallInt"]
pub enum EdgeType {
    Open = 0,
    Wall = 1,
}

impl<DB> ToSql<SmallInt, DB> for EdgeType
where
    DB: Backend,
    i16: ToSql<SmallInt, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        (*self as i16).to_sql(out)
    }
}

impl<DB> FromSql<SmallInt, DB> for EdgeType
where
    DB: Backend,
    i16: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> Result<Self> {
        match i16::from_sql(bytes)? {
            0 => Ok(EdgeType::Open),
            1 => Ok(EdgeType::Wall),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

impl From<i32> for EdgeType {
    fn from(item: i32) -> EdgeType {
        if item == 0 {
            EdgeType::Open
        } else {
            EdgeType::Wall
        }
    }
}

impl From<i16> for EdgeType {
    fn from(item: i16) -> EdgeType {
        if item == 0 {
            EdgeType::Open
        } else {
            EdgeType::Wall
        }
    }
}

impl From<EdgeType> for i32 {
    fn from(item: EdgeType) -> i32 {
        match item {
            EdgeType::Open => 0,
            EdgeType::Wall => 1,
        }
    }
}

impl From<EdgeType> for i16 {
    fn from(item: EdgeType) -> i16 {
        match item {
            EdgeType::Open => 0,
            EdgeType::Wall => 1,
        }
    }
}

#[cfg(test)]
#[test]
fn convert_int_and_edge_type() {
    let e0: EdgeType = 0.into();
    assert_eq!(e0, EdgeType::Open);

    let e1: EdgeType = 1.into();
    assert_eq!(e1, EdgeType::Wall);

    let i0: i32 = EdgeType::Open.into();
    assert_eq!(i0, 0);

    let i1: i32 = EdgeType::Wall.into();
    assert_eq!(i1, 1);
}
