use std::ops::Index;

use {
    super::uml_fn::UMLFn,
    super::{UMLClass},
    super::{UMLRelation, UMLRelationKind},
};


pub struct UMLGraph {
    pub structs: Vec<(String, UMLClass)>,
    pub fns: Vec<UMLFn>,
    relations: Vec<UMLRelation>
}

impl UMLGraph {
    pub fn new() -> UMLGraph {
        UMLGraph { structs: vec![], fns: vec![], relations: vec![]}
    }

    pub fn add_relations(&mut self, rel_list: &mut Vec<UMLRelation>) -> () {
        self.relations.append(rel_list);
    }

    pub fn add_structs(&mut self, st_list: Vec<UMLClass>) -> () {
        for st in st_list {
            if self.get_struct_names().contains(&st.name) {
                println!("struct or trait with name {} exists!", st.name);
            } else {
                let st_name = st.name.clone();
                self.structs.push((st_name.clone(), st));
            }
        }
    }

    pub fn add_impl_classes(&mut self, ip_list: Vec<UMLClass>) -> () {
        for mut ip in ip_list {
            if self.get_struct_names().contains(&ip.name) {
                self.get_mut_struct(&ip.name).unwrap().merge_from(&mut ip);
            } else {
                println!("no struct or trait with name: {}", ip.name);
            }
        }
    }

    pub fn get_relations(&self) -> Vec<UMLRelation> {
        let mut relations = self.relations.clone();

        // compare two adjacent relation, if they have same "from" and "to", then the less ordered Relation will not count in
        relations.sort();
        relations.reverse();
        let mut results: Vec<UMLRelation> = vec![];
        for r in relations {
            match results.last() {
                Some(r_other) => if !r.same_objects(r_other) {
                    results.push(r);
                },
                None => { results.push(r) }
            }
        }

        // if relation's "from" or "to" not in structs/fns, then drop it
        let mut final_results: Vec<UMLRelation> = vec![];
        for r in results {
            if (self.get_fn_names().contains(&r.from) || self.get_struct_names().contains(&r.from)) &&
                (self.get_fn_names().contains(&r.to) || self.get_struct_names().contains(&r.to)) &&
                (&r.from != &r.to) {
                    final_results.push(r);
                }
        }
        
        self.merge_association(final_results)
    }

    fn merge_association(&self, relations: Vec<UMLRelation>) -> Vec<UMLRelation> {
        let mut results = vec![];
        // temp vec for storing association relations
        let mut uni_associations: Vec<UMLRelation> = vec![];
        for r in relations {
            match r.kind {
                // compare relation with Uni Association Type with every Relation in uni_associations,
                // if match with opposite relation, push Bi-Association to Results and remove matched relation from uni_associations,
                // if not, push the relation to uni_associations
                UMLRelationKind::UMLAssociationUni => {
                    let mut match_bi_index: Option<usize> = None;
                    for ua_index in 0..uni_associations.len() {
                        if r.opposite_objects(uni_associations.index(ua_index)) {
                            match_bi_index = Some(ua_index);
                            break;
                        }
                    }
                    match match_bi_index {
                        Some(i) => {
                            results.push(UMLRelation::new(&r.from, &r.to, UMLRelationKind::UMLAssociationBi));
                            uni_associations.remove(i);
                        },
                        None => {
                            uni_associations.push(r);
                        }
                    }
                },
                _ => { results.push(r) }
            }
        }

        // finally merge uni_associations to include unmatched association relations
        results.append(&mut uni_associations);
        results
    }

    fn get_mut_struct(&mut self, struct_name: &str) -> Option<&mut UMLClass> {
        match self.structs.iter_mut().find(|(st_name, _)| st_name == struct_name) {
            Some((_, c)) => Some(c),
            None => None
        }
    }

    fn get_struct_names(&self) -> Vec<String> {
        self.structs
            .iter()
            .map(|(st_name, _)| st_name.clone())
            .collect()
    }

    fn get_fn_names(&self) -> Vec<String> {
        self.fns
            .iter()
            .map(|f| f.name.clone())
            .collect()
    }
}