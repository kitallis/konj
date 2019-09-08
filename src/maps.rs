use indexmap::IndexMap;

pub fn invert(map: &IndexMap<&'static str, &'static str>) -> IndexMap<&'static str, &'static str> {
    let mut invert: IndexMap<&'static str, &'static str> = IndexMap::new();

    for (key, value) in map.into_iter() {
        invert.insert(value, key);
    }

    return invert;
}
