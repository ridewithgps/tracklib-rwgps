use super::rust::simplify_points;
use crate::geometry::{reader_to_points, IrrelevantPointsBehavior};
use crate::polyline::ruby::PolylineOptions;
use crate::polyline::rust::polyline_encode;
use crate::surface::ruby::RubySurfaceMapping;
use itertools::Itertools;
use rutie::{methods, module, AnyObject, Array, Class, Float, Hash, Integer, NilClass, Object, RString, VM};
use std::collections::HashSet;
use tracklib::read::section::SectionRead;

module!(TracklibRwgps);

methods!(
    TracklibRwgps,
    _rtself,
    fn simplify_section_data_simplified_polyline(
        track_reader: ruby_tracklib::read::TrackReader,
        index: Integer,
        mapping: RubySurfaceMapping,
        tolerance: Float,
        polyline_opts: PolylineOptions,
        key_material: RString) -> RString {
        let ruby_track_reader = track_reader.map_err(VM::raise_ex).unwrap();

        let rust_tolerance = tolerance.map_err(VM::raise_ex).unwrap().to_f64();

        let ruby_index = index.map_err(VM::raise_ex).unwrap();
        let rust_index = usize::try_from(ruby_index.to_u64())
            .map_err(|_| VM::raise(Class::from_existing("Exception"), "u64 != usize"))
            .unwrap();

        let ruby_polyline_opts = polyline_opts.map_err(VM::raise_ex).unwrap();
        let rust_polyline_opts = ruby_polyline_opts.inner();

        let ruby_mapping = mapping.map_err(VM::raise_ex).unwrap();
        let rust_mapping = ruby_mapping.inner();

        ruby_track_reader.with_track_reader(|track_reader| {
            track_reader
                .section(rust_index)
                .map(|section| {
                    let schema = tracklib::schema::Schema::with_fields(vec![
                        tracklib::schema::FieldDefinition::new("x", tracklib::schema::DataType::F64 { scale: 6 }),
                        tracklib::schema::FieldDefinition::new("y", tracklib::schema::DataType::F64 { scale: 6 }),
                        tracklib::schema::FieldDefinition::new("e", tracklib::schema::DataType::F64 { scale: 1 }),
                        tracklib::schema::FieldDefinition::new("S", tracklib::schema::DataType::U64),
                        tracklib::schema::FieldDefinition::new("R", tracklib::schema::DataType::U64),
                    ]);

                    match section {
                        tracklib::read::section::Section::Standard(section) => {
                            let section_reader = section
                                .reader_for_schema(&schema)
                                .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                .unwrap();
                            let points = reader_to_points(section_reader, IrrelevantPointsBehavior::Ignore);
                            let simplified_indexes = simplify_points(&points, &rust_mapping, rust_tolerance);
                            let simplified_points = simplified_indexes
                                .into_iter()
                                .sorted()
                                .map(|index| points[index].clone())
                                .collect::<Vec<_>>();

                            RString::from(polyline_encode(&simplified_points, rust_polyline_opts))
                        }
                        tracklib::read::section::Section::Encrypted(mut section) => {
                            let ruby_key_material = key_material.map_err(VM::raise_ex).unwrap();
                            let rust_key_material = ruby_key_material.to_bytes_unchecked();

                            let section_reader = section
                                .reader_for_schema(rust_key_material, &schema)
                                .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                .unwrap();
                            let points = reader_to_points(section_reader, IrrelevantPointsBehavior::Ignore);
                            let simplified_indexes = simplify_points(&points, &rust_mapping, rust_tolerance);
                            let simplified_points = simplified_indexes
                                .into_iter()
                                .sorted()
                                .map(|index| points[index].clone())
                                .collect::<Vec<_>>();

                            RString::from(polyline_encode(&simplified_points, rust_polyline_opts))
                        }
                    }
                })
                .ok_or_else(|| VM::raise(Class::from_existing("Exception"), "Section does not exist"))
                .unwrap()
        })
    },
    fn simplify_section_data_simplified(
        track_reader: ruby_tracklib::read::TrackReader,
        index: Integer,
        mapping: RubySurfaceMapping,
        tolerance: Float,
        key_material: RString) -> Array {
        let ruby_track_reader = track_reader.map_err(VM::raise_ex).unwrap();

        let rust_tolerance = tolerance.map_err(VM::raise_ex).unwrap().to_f64();

        let ruby_index = index.map_err(VM::raise_ex).unwrap();
        let rust_index = usize::try_from(ruby_index.to_u64())
            .map_err(|_| VM::raise(Class::from_existing("Exception"), "u64 != usize"))
            .unwrap();

        let ruby_mapping = mapping.map_err(VM::raise_ex).unwrap();
        let rust_mapping = ruby_mapping.inner();

        ruby_track_reader.with_track_reader(|track_reader| {
            track_reader
                .section(rust_index)
                .map(|section| {
                    let schema = tracklib::schema::Schema::with_fields(vec![
                        tracklib::schema::FieldDefinition::new("x", tracklib::schema::DataType::F64 { scale: 6 }),
                        tracklib::schema::FieldDefinition::new("y", tracklib::schema::DataType::F64 { scale: 6 }),
                        tracklib::schema::FieldDefinition::new("e", tracklib::schema::DataType::F64 { scale: 1 }),
                        tracklib::schema::FieldDefinition::new("S", tracklib::schema::DataType::U64),
                        tracklib::schema::FieldDefinition::new("R", tracklib::schema::DataType::U64),
                    ]);

                    match section {
                        tracklib::read::section::Section::Standard(section) => {
                            let section_reader_for_simplification = section
                                .reader_for_schema(&schema)
                                .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                .unwrap();
                            let points =
                                reader_to_points(section_reader_for_simplification, IrrelevantPointsBehavior::Count);
                            let simplified_indexes = simplify_points(&points, &rust_mapping, rust_tolerance);

                            let section_reader_for_serialization = section
                                .reader()
                                .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                .unwrap();

                            reader_with_indexes_to_array_of_hashes(
                                section_reader_for_serialization,
                                &simplified_indexes,
                            )
                        }
                        tracklib::read::section::Section::Encrypted(mut section) => {
                            let ruby_key_material = key_material.map_err(VM::raise_ex).unwrap();
                            let rust_key_material = ruby_key_material.to_bytes_unchecked();

                            let section_reader_for_simplification = section
                                .reader_for_schema(rust_key_material, &schema)
                                .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                .unwrap();
                            let points =
                                reader_to_points(section_reader_for_simplification, IrrelevantPointsBehavior::Count);
                            let simplified_indexes = simplify_points(&points, &rust_mapping, rust_tolerance);

                            let section_reader_for_serialization = section
                                .reader(rust_key_material)
                                .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                .unwrap();

                            reader_with_indexes_to_array_of_hashes(
                                section_reader_for_serialization,
                                &simplified_indexes,
                            )
                        }
                    }
                })
                .ok_or_else(|| VM::raise(Class::from_existing("Exception"), "Section does not exist"))
                .unwrap()
        })
    },
    fn simplify_section_column_simplified(
        track_reader: ruby_tracklib::read::TrackReader,
        index: Integer,
        column_name: RString,
        mapping: RubySurfaceMapping,
        tolerance: Float,
        key_material: RString) -> AnyObject {
        let ruby_track_reader = track_reader.map_err(VM::raise_ex).unwrap();

        let rust_tolerance = tolerance.map_err(VM::raise_ex).unwrap().to_f64();

        let ruby_index = index.map_err(VM::raise_ex).unwrap();
        let rust_index = usize::try_from(ruby_index.to_u64())
            .map_err(|_| VM::raise(Class::from_existing("Exception"), "u64 != usize"))
            .unwrap();

        let ruby_mapping = mapping.map_err(VM::raise_ex).unwrap();
        let rust_mapping = ruby_mapping.inner();

        ruby_track_reader.with_track_reader(|track_reader| {
            track_reader
                .section(rust_index)
                .map(|section| {
                    let ruby_field_name = column_name.map_err(VM::raise_ex).unwrap();
                    let field_name = ruby_field_name.to_str();

                    let schema = match section {
                        tracklib::read::section::Section::Standard(ref section) => section.schema(),
                        tracklib::read::section::Section::Encrypted(ref section) => section.schema(),
                    };
                    let maybe_field_def = schema.fields().iter().find(|field_def| field_def.name() == field_name);

                    if let Some(field_def) = maybe_field_def {
                        let schema_for_serialization = tracklib::schema::Schema::with_fields(vec![field_def.clone()]);

                        let schema_for_simplification = tracklib::schema::Schema::with_fields(vec![
                            tracklib::schema::FieldDefinition::new("x", tracklib::schema::DataType::F64 { scale: 6 }),
                            tracklib::schema::FieldDefinition::new("y", tracklib::schema::DataType::F64 { scale: 6 }),
                            tracklib::schema::FieldDefinition::new("e", tracklib::schema::DataType::F64 { scale: 1 }),
                            tracklib::schema::FieldDefinition::new("S", tracklib::schema::DataType::U64),
                            tracklib::schema::FieldDefinition::new("R", tracklib::schema::DataType::U64),
                        ]);

                        match section {
                            tracklib::read::section::Section::Standard(section) => {
                                let section_reader_for_simplification = section
                                    .reader_for_schema(&schema_for_simplification)
                                    .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                    .unwrap();
                                let points = reader_to_points(
                                    section_reader_for_simplification,
                                    IrrelevantPointsBehavior::Count,
                                );
                                let simplified_indexes = simplify_points(&points, &rust_mapping, rust_tolerance);

                                let section_reader_for_serialization = section
                                    .reader_for_schema(&schema_for_serialization)
                                    .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                    .unwrap();
                                reader_with_indexes_to_single_column_array(
                                    section_reader_for_serialization,
                                    &simplified_indexes,
                                )
                                .to_any_object()
                            }
                            tracklib::read::section::Section::Encrypted(mut section) => {
                                let ruby_key_material = key_material.map_err(VM::raise_ex).unwrap();
                                let rust_key_material = ruby_key_material.to_bytes_unchecked();

                                let section_reader_for_simplification = section
                                    .reader_for_schema(rust_key_material, &schema)
                                    .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                    .unwrap();
                                let points = reader_to_points(
                                    section_reader_for_simplification,
                                    IrrelevantPointsBehavior::Count,
                                );
                                let simplified_indexes = simplify_points(&points, &rust_mapping, rust_tolerance);

                                let section_reader_for_serialization = section
                                    .reader_for_schema(rust_key_material, &schema_for_serialization)
                                    .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                    .unwrap();
                                reader_with_indexes_to_single_column_array(
                                    section_reader_for_serialization,
                                    &simplified_indexes,
                                )
                                .to_any_object()
                            }
                        }
                    } else {
                        NilClass::new().to_any_object()
                    }
                })
                .ok_or_else(|| VM::raise(Class::from_existing("Exception"), "Section does not exist"))
                .unwrap()
        })
    }
);

fn reader_with_indexes_to_array_of_hashes(
    mut reader: tracklib::read::section::reader::SectionReader,
    indexes: &HashSet<usize>,
) -> Array {
    let mut data_array = Array::new();
    let mut i = 0;
    while let Some(columniter) = reader.open_column_iter() {
        if indexes.contains(&i) {
            let mut row_hash = Hash::new();
            for row in columniter {
                let (field_def, maybe_value) = row
                    .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                    .unwrap();

                if let Some(value) = maybe_value {
                    row_hash.store(
                        RString::from(String::from(field_def.name())),
                        ruby_tracklib::read::fieldvalue_to_ruby(value),
                    );
                }
            }
            data_array.push(row_hash);
        } else {
            columniter.for_each(drop); // fully consume (and ignore) this row
        }
        i += 1;
    }

    data_array
}

fn reader_with_indexes_to_single_column_array(
    mut reader: tracklib::read::section::reader::SectionReader,
    indexes: &HashSet<usize>,
) -> Array {
    let mut data_array = Array::new();
    let mut i = 0;
    while let Some(mut columniter) = reader.open_column_iter() {
        if indexes.contains(&i) {
            let (_field_def, maybe_value) = columniter
                .next()
                .ok_or_else(|| VM::raise(Class::from_existing("Exception"), "Missing field inside iterator"))
                .unwrap()
                .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                .unwrap();

            let ruby_value = if let Some(value) = maybe_value {
                ruby_tracklib::read::fieldvalue_to_ruby(value)
            } else {
                NilClass::new().to_any_object()
            };

            data_array.push(ruby_value);
        } else {
            columniter.for_each(drop); // fully consume (and ignore) this row
        }
        i += 1;
    }

    data_array
}
