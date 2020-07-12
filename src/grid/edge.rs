#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EdgeType {
    Open,
    Wall,
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