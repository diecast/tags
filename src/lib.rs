extern crate diecast;
extern crate typemap;

use std::collections::HashMap;
use std::sync::Arc;

use diecast::{Bind, Item, Handle};

#[derive(Clone)]
pub struct Tag {
    pub tag: String,
    pub items: Arc<Vec<Arc<Item>>>,
}

impl typemap::Key for Tag {
    type Value = Tag;
}

pub struct Tags;

impl typemap::Key for Tags {
    type Value = HashMap<String, Arc<Vec<Arc<Item>>>>;
}

pub struct Collector<H>
where H: Fn(&Item) -> Vec<String> {
    collect: H,
}

impl<H> Handle<Bind> for Collector<H>
where H: Fn(&Item) -> Vec<String> {
    fn handle(&self, bind: &mut Bind) -> diecast::Result<()> {
        let mut tag_map = ::std::collections::HashMap::new();

        for item in bind.iter() {
            let arc = Arc::new(item.clone());

            let tags = (self.collect)(item);

            for tag in tags {
                tag_map.entry(tag)
                    .or_insert(vec![])
                    .push(arc.clone());
            }
        }

        let mut arc_map = HashMap::new();

        for (k, v) in tag_map {
            arc_map.insert(k, Arc::new(v));
        }

        bind.extensions.write().unwrap().insert::<Tags>(arc_map);

        Ok(())
    }
}

pub fn collect<H>(handler: H) -> Collector<H>
where H: Fn(&Item) -> Vec<String> {
    Collector { collect: handler }
}

