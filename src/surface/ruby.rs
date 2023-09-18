use super::rust::{RoadClassMapping, SurfaceMapping};
use rutie::{
    class, methods, wrappable_struct, AnyObject, Array, Class, Float, Integer, Module, NilClass, Object, RString,
    VerifiedObject, VM,
};

pub struct RoadClassInner {
    inner: RoadClassMapping,
}

wrappable_struct!(RoadClassInner, RoadClassInnerWrapper, ROAD_CLASS_INNER_WRAPPER);

class!(RubyRoadClassMapping);

methods!(
    RubyRoadClassMapping,
    itself,
    fn road_class_mapping_new(bbox_array: Array) -> AnyObject {
        let bbox_vec: Vec<f64> = bbox_array
            .map_err(|e| VM::raise_ex(e))
            .unwrap()
            .into_iter()
            .map(|ele| match ele.try_convert_to::<Float>() {
                Ok(f) => f.to_f64(),
                Err(float_e) => ele
                    .try_convert_to::<Integer>()
                    .map_err(|_| VM::raise_ex(float_e))
                    .unwrap()
                    .to_i32()
                    .into(),
            })
            .collect();
        if bbox_vec.len() != 4 {
            VM::raise(Class::from_existing("Exception"), "BBOX Array len must be 4");
        }
        let bbox = [bbox_vec[0], bbox_vec[1], bbox_vec[2], bbox_vec[3]];

        let inner = RoadClassInner {
            inner: crate::surface::rust::RoadClassMapping::new(bbox),
        };

        Module::from_existing("TracklibRwgps")
            .get_nested_class("RoadClassMapping")
            .wrap_data(inner, &*ROAD_CLASS_INNER_WRAPPER)
    },
    fn road_class_mapping_add_road_class(road_class_id: Integer, surface_id: Integer) -> NilClass {
        let rc_id = road_class_id.map_err(|e| VM::raise_ex(e)).unwrap().to_u64();
        let s_id = surface_id.map_err(|e| VM::raise_ex(e)).unwrap().to_u64();
        let mapping = &mut itself.get_data_mut(&*ROAD_CLASS_INNER_WRAPPER).inner;
        mapping.add_road_class(rc_id, s_id);

        NilClass::new()
    },
    fn road_class_mapping_to_s() -> RString {
        let mapping = &itself.get_data(&*ROAD_CLASS_INNER_WRAPPER).inner;

        RString::new_utf8(&format!("{:?}", mapping))
    }
);

impl VerifiedObject for RubyRoadClassMapping {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class() == Module::from_existing("TracklibRwgps").get_nested_class("RoadClassMapping")
    }

    fn error_message() -> &'static str {
        "Error converting to RoadClassMapping"
    }
}

pub struct SurfaceInner {
    inner: SurfaceMapping,
}

wrappable_struct!(SurfaceInner, SurfaceInnerWrapper, SURFACE_INNER_WRAPPER);

class!(RubySurfaceMapping);

methods!(
    RubySurfaceMapping,
    itself,
    fn surface_mapping_new(unknown_surface_id: Integer) -> AnyObject {
        let id = unknown_surface_id.map_err(|e| VM::raise_ex(e)).unwrap().to_u64();
        let inner = SurfaceInner {
            inner: crate::surface::rust::SurfaceMapping::new(id),
        };

        Module::from_existing("TracklibRwgps")
            .get_nested_class("SurfaceMapping")
            .wrap_data(inner, &*SURFACE_INNER_WRAPPER)
    },
    fn surface_mapping_add_surface(surface_id: Integer, group: RString) -> NilClass {
        let id = surface_id.map_err(|e| VM::raise_ex(e)).unwrap().to_u64();
        let group_name = group.map_err(|e| VM::raise_ex(e)).unwrap().to_string();
        let mapping = &mut itself.get_data_mut(&*SURFACE_INNER_WRAPPER).inner;
        mapping.add_surface(id, group_name);

        NilClass::new()
    },
    fn surface_mapping_add_road_class_mapping(road_class_mapping: RubyRoadClassMapping) -> NilClass {
        let rcm = road_class_mapping.map_err(|e| VM::raise_ex(e)).unwrap();
        let mapping = &mut itself.get_data_mut(&*SURFACE_INNER_WRAPPER).inner;
        let road_class = &rcm.get_data(&*ROAD_CLASS_INNER_WRAPPER).inner;
        mapping.add_road_class_mapping(road_class.clone());

        NilClass::new()
    },
    fn surface_mapping_to_s() -> RString {
        let mapping = &itself.get_data(&*SURFACE_INNER_WRAPPER).inner;

        RString::new_utf8(&format!("{:?}", mapping))
    }
);

impl RubySurfaceMapping {
    pub fn inner(&self) -> &SurfaceMapping {
        &self.get_data(&*SURFACE_INNER_WRAPPER).inner
    }
}

impl VerifiedObject for RubySurfaceMapping {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class() == Module::from_existing("TracklibRwgps").get_nested_class("SurfaceMapping")
    }

    fn error_message() -> &'static str {
        "Error converting to SurfaceMapping"
    }
}
