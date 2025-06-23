use crate::models::{archivable::Archived, class::Class, output_data::OutputData, types::Type};

/// A single resolved property: either
///  - an object with its class and fully-resolved children,
///  - a group of mixed properties,
///  - or a primitive value.
#[derive(Debug)]
pub enum ResolvedProperty<'a, 'b> {
    Object {
        class: &'a Class,
        name: &'a str,
        data: PropertyResolverIterator<'a, 'b>,
    },
    Group(Vec<ResolvedProperty<'a, 'b>>),
    Primitive(&'b OutputData<'a>),
}
/// An iterator that resolves the top-level properties of a single [`Archived::Object`].
///
/// It iterates over the `Vec<Vec<OutputData>>` of an object. If an inner `Vec`
/// contains a single `OutputData::Object` reference, it resolves that reference
/// against the `object_table` and yields a `ResolvedProperty::Object`. Otherwise,
/// it yields a `ResolvedProperty::Data` containing the slice of properties.
#[derive(Debug)]
pub struct PropertyResolverIterator<'a, 'b> {
    object_table: &'b [Archived<'a>],
    type_table: &'b [Vec<Type<'a>>],
    property_groups: std::slice::Iter<'b, Vec<OutputData<'a>>>,
}

impl<'a, 'b> PropertyResolverIterator<'a, 'b> {
    pub(crate) fn new(
        object_table: &'b [Archived<'a>],
        type_table: &'b [Vec<Type<'a>>],
        root_object_index: usize,
    ) -> Option<Self> {
        let root_object = object_table.get(root_object_index)?;

        let properties = if let Archived::Object { data, .. } = root_object {
            data
        } else {
            return None;
        };

        Some(Self {
            object_table,
            type_table,
            property_groups: properties.iter(),
        })
    }
}

impl<'a, 'b: 'a> Iterator for PropertyResolverIterator<'a, 'b> {
    type Item = ResolvedProperty<'a, 'b>;

    fn next(&mut self) -> Option<Self::Item> {
        let groups = self.property_groups.next()?;

        let mut resolved = Vec::with_capacity(groups.len());

        for group in groups {
            match group {
                OutputData::Object(idx) => {
                    if let Some(Archived::Object {
                        class: cls,
                        data: _,
                    }) = self.object_table.get(*idx)
                    {
                        if let Some(Archived::Class(cls)) = self.object_table.get(*cls) {
                            let class_name = self
                                .type_table
                                .get(cls.name_index)
                                .and_then(|types| types.first())
                                .and_then(|t| match t {
                                    Type::String(name) => Some(*name),
                                    _ => None,
                                })
                                .unwrap_or("Unknown Class");
                            // recurse into that object’s own data
                            let sub_iter = PropertyResolverIterator::new(
                                self.object_table,
                                self.type_table,
                                *idx,
                            )?;
                            resolved.push(ResolvedProperty::Object {
                                class: cls,
                                name: class_name,
                                data: sub_iter,
                            });
                        }
                    }
                }
                prim => resolved.push(ResolvedProperty::Primitive(prim)),
            }
        }
        Some(ResolvedProperty::Group(resolved))
    }
}

/// Walk an entire `PropertyResolverIterator`, printing each property
/// with `indent` spaces of indentation.
pub fn print_resolved<'a, 'b>(iter: PropertyResolverIterator<'a, 'b>, indent: usize) {
    for prop in iter {
        print_property(prop, indent);
    }
}

/// Recursively walk a `PropertyResolverIterator` and print each property
/// with `indent` spaces of indentation.
pub fn print_property<'a, 'b: 'a>(prop: ResolvedProperty<'a, 'b>, indent: usize) {
    match prop {
        ResolvedProperty::Object {
            class: _,
            name,
            data,
        } => {
            // Print the object itself
            println!("{:indent$}Object: {:?}", "", name, indent = indent);
            // Recurse into its children with increased indent
            print_resolved(data, indent + 2);
        }
        ResolvedProperty::Group(slice) => {
            println!("{:indent$}Group:", "", indent = indent);
            // drill into every slot in the group
            for slot in slice {
                print_property(slot, indent + 2);
            }
        }
        ResolvedProperty::Primitive(p) => {
            println!("{:indent$}Primitive: {:?}", "", p, indent = indent);
        }
    }
}
