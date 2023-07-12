mod geometry;
mod polyline;
mod simplify;
mod surface;

use rutie::{Module, Object};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_Tracklib_Rwgps() {
    Module::from_existing("TracklibRwgps").define(|module| {
        module.define_nested_class("PolylineOptions", None).define(|class| {
            class.def_self("new", polyline::ruby::polyline_options_new);
        });

        module.define_nested_class("RoadClassMapping", None).define(|class| {
            class.def_self("new", surface::ruby::road_class_mapping_new);
            class.def("add_road_class", surface::ruby::road_class_mapping_add_road_class);
            class.def("to_s", surface::ruby::road_class_mapping_to_s);
        });

        module.define_nested_class("SurfaceMapping", None).define(|class| {
            class.def_self("new", surface::ruby::surface_mapping_new);
            class.def("add_surface", surface::ruby::surface_mapping_add_surface);
            class.def(
                "add_road_class_mapping",
                surface::ruby::surface_mapping_add_road_class_mapping,
            );
            class.def("to_s", surface::ruby::surface_mapping_to_s);
        });

        module.define_module_function("section_data_polyline", polyline::ruby::polyline_section_data_polyline);
        module.define_module_function(
            "section_data_simplified_polyline",
            simplify::ruby::simplify_section_data_simplified_polyline,
        );
        module.define_module_function(
            "section_data_simplified",
            simplify::ruby::simplify_section_data_simplified,
        );
        module.define_module_function(
            "section_column_simplified",
            simplify::ruby::simplify_section_column_simplified,
        );
    });
}
