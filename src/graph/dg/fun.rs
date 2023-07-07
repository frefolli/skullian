use crate::graph::dg::{defkind::Defkind, edge_label::EdgeLabel};

use super::dep_graph::DepGraph;

fn fun_facts_about_nodes(dep_graph: &DepGraph) {
    let mut packages = 0;
    let mut classes = 0;
    let mut interfaces = 0;
    let mut functions = 0;
    let mut parameters = 0;
    let mut attributes = 0;
    let mut enums = 0;
    let mut enum_variants = 0;
    let mut constants = 0;
    let mut annotations = 0;
    let mut annotation_elements = 0;
    let mut files = 0;
    let mut others = 0;

    for (_node, _data) in dep_graph.iter_nodes() {
        match _data.get_defkind() {
            Defkind::Package => packages += 1,
            Defkind::Class => classes += 1,
            Defkind::Interface => interfaces += 1,
            Defkind::Function => functions += 1,
            Defkind::Parameter => parameters += 1,
            Defkind::Attribute => attributes += 1,
            Defkind::Enum => enums += 1,
            Defkind::EnumVariant => enum_variants += 1,
            Defkind::Nothing => others += 1,
            Defkind::Constant => constants += 1,
            Defkind::Annotation => annotations += 1,
            Defkind::AnnotationElement => annotation_elements += 1,
            Defkind::File => files += 1
        }
    }

    let total = packages + classes +
                     interfaces + functions +
                     parameters + attributes +
                     enums + enum_variants +
                     constants + annotations +
                     annotation_elements + files + others;
    log::info!("found {} packages", packages);
    log::info!("found {} classes", classes);
    log::info!("found {} interfaces", interfaces);
    log::info!("found {} functions", functions);
    log::info!("found {} parameters", parameters);
    log::info!("found {} attributes", attributes);
    log::info!("found {} enums", enums);
    log::info!("found {} enum_variants", enum_variants);
    log::info!("found {} constants", constants);
    log::info!("found {} annotations", annotations);
    log::info!("found {} annotation_elements", annotation_elements);
    log::info!("found {} files", files);
    log::info!("found {} other nodes", others);
    log::info!("total: {} nodes", total);
}

fn fun_facts_about_edges(dep_graph: &DepGraph) {
    let mut defined_by = 0;
    let mut is_implementation_of = 0;
    let mut is_child_of = 0;
    let mut nested_to = 0;
    let mut includes = 0;
    let mut uses_type = 0;
    let mut access_field = 0;
    let mut calls = 0;
    let mut casts_type = 0;
    let mut throws_type = 0;

    for (_node, _edges) in dep_graph.iter_edges() {
        for edge in _edges.iter() {
            match edge.get_label() {
                EdgeLabel::DefinedBy => defined_by += 1,
                EdgeLabel::IsImplementationOf => is_implementation_of += 1,
                EdgeLabel::IsChildOf => is_child_of += 1,
                EdgeLabel::NestedTo => nested_to += 1,
                EdgeLabel::Includes => includes += 1,
                EdgeLabel::UsesType => uses_type += 1,
                EdgeLabel::AccessField => access_field += 1,
                EdgeLabel::Calls => calls += 1,
                EdgeLabel::CastsType => casts_type += 1,
                EdgeLabel::ThrowsType => throws_type += 1
            }
        }
    }

    let total = defined_by + is_implementation_of +
                     is_child_of + nested_to +
                     includes + uses_type +
                     access_field + calls +
                     casts_type + throws_type;
    log::info!("found {} definedBy", defined_by);
    log::info!("found {} isImplementationOf", is_implementation_of);
    log::info!("found {} isChildOf", is_child_of);
    log::info!("found {} nestedTo", nested_to);
    log::info!("found: {} includes", includes);
    log::info!("found: {} uses_type", uses_type);
    log::info!("found: {} access_field", access_field);
    log::info!("found: {} calls", calls);
    log::info!("found: {} casts_type", casts_type);
    log::info!("found: {} throws_type", throws_type);
    log::info!("total: {} edges", total);
}

pub fn fun_facts(dep_graph: &DepGraph) {
    fun_facts_about_nodes(dep_graph);
    fun_facts_about_edges(dep_graph);
}