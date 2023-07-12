require "spec_helper"

describe tracklibRwgps do
  context "Surface Type Mappings" do
    it "can be created without throwing an exception" do
      sm = tracklibRwgps::SurfaceMapping::new(99)
      sm.add_surface(0, "Paved")
      sm.add_surface(1, "Paved")
      sm.add_surface(20, "Gravel")
      sm.add_surface(25, "Gravel")

      # now add a RoadClassMapping
      rcm = tracklibRwgps::RoadClassMapping::new([-90, -180, 90, 180])
      rcm.add_road_class(0, 10)
      rcm.add_road_class(1, 20)

      sm.add_road_class_mapping(rcm)
    end
  end
end
