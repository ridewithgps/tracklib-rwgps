use super::rust::{polyline_encode, PointField, PolylineOption};
use crate::geometry::reader_to_points;
use rutie::{
    class, methods, module, wrappable_struct, AnyObject, Array, Class, Integer, Module, Object, RString,
    VerifiedObject, VM,
};

pub struct WrappablePolylineOptions {
    opts: Vec<PolylineOption>,
}

wrappable_struct!(
    WrappablePolylineOptions,
    PolylineOptionsWrapper,
    POLYLINE_OPTIONS_WRAPPER_INSTANCE
);

class!(PolylineOptions);

methods!(
    PolylineOptions,
    rtself,
    fn polyline_options_new(ruby_opts: Array) -> AnyObject {
        let opts = ruby_opts
            .map_err(VM::raise_ex)
            .unwrap()
            .into_iter()
            .map(|ele| {
                let ruby_opt = ele.try_convert_to::<Array>().map_err(VM::raise_ex).unwrap();

                let ruby_field_name = ruby_opt
                    .at(0)
                    .try_convert_to::<RString>()
                    .map_err(VM::raise_ex)
                    .unwrap();
                let ruby_precision = ruby_opt
                    .at(1)
                    .try_convert_to::<Integer>()
                    .map_err(VM::raise_ex)
                    .unwrap();
                let ruby_default_value = ruby_opt.at(2);

                let precision = ruby_precision.to_u32();
                let factor = f64::from(10_u32.pow(precision));

                let field = match ruby_field_name.to_str() {
                    "y" => {
                        // disallow a default value (third array element)
                        if !ruby_default_value.is_nil() {
                            VM::raise(
                                Class::from_existing("Exception"),
                                "Polyline parameter 'y' does not allow a default value",
                            );
                        }
                        PointField::Y
                    }
                    "x" => {
                        // disallow a default value (third array element)
                        if !ruby_default_value.is_nil() {
                            VM::raise(
                                Class::from_existing("Exception"),
                                "Polyline parameter 'x' does not allow a default value",
                            );
                        }
                        PointField::X
                    }
                    "d" => {
                        // disallow a default value (third array element)
                        if !ruby_default_value.is_nil() {
                            VM::raise(
                                Class::from_existing("Exception"),
                                "Polyline parameter 'd' does not allow a default value",
                            );
                        }
                        PointField::D
                    }
                    "e" => {
                        // disallow a default value (third array element)
                        if !ruby_default_value.is_nil() {
                            VM::raise(
                                Class::from_existing("Exception"),
                                "Polyline parameter 'e' does not allow a default value",
                            );
                        }
                        PointField::E
                    }
                    "S" => {
                        if ruby_default_value.is_nil() {
                            VM::raise(
                                Class::from_existing("Exception"),
                                "Polyline parameter 'S' requires a default value",
                            );
                        }
                        let default = ruby_default_value
                            .try_convert_to::<Integer>()
                            .map_err(VM::raise_ex)
                            .unwrap()
                            .to_i64();
                        PointField::S { default }
                    }
                    "R" => {
                        if ruby_default_value.is_nil() {
                            VM::raise(
                                Class::from_existing("Exception"),
                                "Polyline parameter 'R' requires a default value",
                            );
                        }
                        let default = ruby_default_value
                            .try_convert_to::<Integer>()
                            .map_err(VM::raise_ex)
                            .unwrap()
                            .to_i64();
                        PointField::R { default }
                    }
                    field_name => {
                        VM::raise(
                            Class::from_existing("Exception"),
                            &format!("Polyline parameter '{field_name}' is not valid"),
                        );
                        unreachable!();
                    }
                };

                PolylineOption::new(field, factor)
            })
            .collect::<Vec<_>>();

        Module::from_existing("TracklibRwgps")
            .get_nested_class("PolylineOptions")
            .wrap_data(WrappablePolylineOptions { opts }, &*POLYLINE_OPTIONS_WRAPPER_INSTANCE)
    },
);

impl PolylineOptions {
    pub(crate) fn inner(&self) -> &[PolylineOption] {
        &self.get_data(&*POLYLINE_OPTIONS_WRAPPER_INSTANCE).opts
    }
}

impl VerifiedObject for PolylineOptions {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class() == Module::from_existing("TracklibRwgps").get_nested_class("PolylineOptions")
    }

    fn error_message() -> &'static str {
        "Error converting to PolylineOptions"
    }
}

module!(TracklibRwgps);

methods!(
    TracklibRwgps,
    _rtself,
    fn polyline_section_data_polyline(
        track_reader: ruby_tracklib::read::TrackReader,
        index: Integer,
        polyline_opts: PolylineOptions,
        key_material: RString) -> RString {
        let ruby_track_reader = track_reader.map_err(VM::raise_ex).unwrap();

        let ruby_index = index.map_err(VM::raise_ex).unwrap();
        let rust_index = usize::try_from(ruby_index.to_u64())
            .map_err(|_| VM::raise(Class::from_existing("Exception"), "u64 != usize"))
            .unwrap();

        let ruby_polyline_opts = polyline_opts.map_err(VM::raise_ex).unwrap();
        let rust_polyline_opts = ruby_polyline_opts.inner();

        ruby_track_reader.with_track_reader(|track_reader| {
            track_reader
                .section(rust_index)
                .map(|section| {
                    let schema = tracklib::schema::Schema::with_fields(vec![
                        tracklib::schema::FieldDefinition::new("x", tracklib::schema::DataType::F64 { scale: 6 }),
                        tracklib::schema::FieldDefinition::new("y", tracklib::schema::DataType::F64 { scale: 6 }),
                        tracklib::schema::FieldDefinition::new("e", tracklib::schema::DataType::F64 { scale: 1 }),
                        tracklib::schema::FieldDefinition::new("S", tracklib::schema::DataType::I64),
                        tracklib::schema::FieldDefinition::new("R", tracklib::schema::DataType::I64),
                    ]);

                    match section {
                        tracklib::read::section::Section::Standard(section) => {
                            let section_reader = section
                                .reader_for_schema(&schema)
                                .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                .unwrap();
                            let points = reader_to_points(section_reader);

                            RString::from(polyline_encode(&points, rust_polyline_opts))
                        }
                        tracklib::read::section::Section::Encrypted(mut section) => {
                            let ruby_key_material = key_material.map_err(VM::raise_ex).unwrap();
                            let rust_key_material = ruby_key_material.to_bytes_unchecked();

                            let section_reader = section
                                .reader_for_schema(rust_key_material, &schema)
                                .map_err(|e| VM::raise(Class::from_existing("Exception"), &format!("{}", e)))
                                .unwrap();
                            let points = reader_to_points(section_reader);

                            RString::from(polyline_encode(&points, rust_polyline_opts))
                        }
                    }
                })
                .ok_or_else(|| VM::raise(Class::from_existing("Exception"), "Section does not exist"))
                .unwrap()
        })
    }
);
