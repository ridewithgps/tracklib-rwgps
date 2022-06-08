require "spec_helper"

describe TracklibRWGPS do
  context "simplify and encode" do
    it "can work in the simple case" do
      data = [{"x" => 40, "y" => 12, "e" => 2},
              {"x" => 41, "y" => 800, "e" => 2}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      surface_mapping = TracklibRWGPS::SurfaceMapping::new(99)

      polyline = TracklibRWGPS::section_data_simplified_polyline(reader,
                                                                 0,
                                                                 surface_mapping,
                                                                 0.0,
                                                                 TracklibRWGPS::PolylineOptions::new([["y", 5], ["x", 5]]))
      expect(decode_polyline(polyline, [5, 5]))
        .to eq([12.0, 40.0, 800.0, 41.0])
    end

    it "can simplify and encode encrypted sections" do
      data = [{"x" => 40, "y" => 12, "e" => 1},
              {"x" => 41, "y" => 800, "e" => 1}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7]])
      key = "01234567890123456789012345678901"
      section = Tracklib::Section::encrypted(schema, data, key)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      surface_mapping = TracklibRWGPS::SurfaceMapping::new(99)

      polyline = TracklibRWGPS::section_data_simplified_polyline(reader,
                                                                 0,
                                                                 surface_mapping,
                                                                 0.0,
                                                                 TracklibRWGPS::PolylineOptions::new([["y", 5], ["x", 5]]),
                                                                 key)

      expect(decode_polyline(polyline, [5, 5]))
        .to eq([12.0, 40.0, 800.0, 41.0])
    end
  end

  context "simplify and serialize" do
    it "can work in the simple case" do
      data = [{"x" => 40, "y" => 12, "e" => 2, "z" => "Foo"},
              {"x" => 41, "y" => 800, "e" => 2}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7], ["z", :string]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      surface_mapping = TracklibRWGPS::SurfaceMapping::new(99)

      new_data = TracklibRWGPS::section_data_simplified(reader,
                                                        0,
                                                        surface_mapping,
                                                        0.0)
      expect(new_data).to eq(data)
    end

    it "can work on encrypted sections" do
      data = [{"x" => 40, "y" => 12, "e" => 2, "z" => "Foo"},
              {"x" => 41, "y" => 800, "e" => 2}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7], ["z", :string]])
      key = "01234567890123456789012345678901"
      section = Tracklib::Section::encrypted(schema, data, key)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      surface_mapping = TracklibRWGPS::SurfaceMapping::new(99)

      new_data = TracklibRWGPS::section_data_simplified(reader,
                                                        0,
                                                        surface_mapping,
                                                        0.0,
                                                        key)
      expect(new_data).to eq(data)
    end
  end

  context "simplify and serialize a single column" do
    it "can work in the simple case" do
      data = [{"x" => 40, "y" => 12, "e" => 2, "z" => "Foo"},
              {"x" => 41, "y" => 800, "e" => 2}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7], ["z", :string]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      surface_mapping = TracklibRWGPS::SurfaceMapping::new(99)

      expect(TracklibRWGPS::section_column_simplified(reader,
                                                      0,
                                                      "x",
                                                      surface_mapping,
                                                      0.0))
        .to eq([40, 41])

      expect(TracklibRWGPS::section_column_simplified(reader,
                                                      0,
                                                      "z",
                                                      surface_mapping,
                                                      0.0))
        .to eq(["Foo", nil])

      expect(TracklibRWGPS::section_column_simplified(reader,
                                                      0,
                                                      "F",
                                                      surface_mapping,
                                                      0.0))
        .to eq(nil)
    end

    it "can work on encrypted sections" do
      data = [{"x" => 40, "y" => 12, "e" => 2, "z" => "Foo"},
              {"x" => 41, "y" => 800, "e" => 2}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7], ["z", :string]])
      key = "01234567890123456789012345678901"
      section = Tracklib::Section::encrypted(schema, data, key)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      surface_mapping = TracklibRWGPS::SurfaceMapping::new(99)

      expect(TracklibRWGPS::section_column_simplified(reader,
                                                      0,
                                                      "x",
                                                      surface_mapping,
                                                      0.0,
                                                      key))
        .to eq([40, 41])

      expect(TracklibRWGPS::section_column_simplified(reader,
                                                      0,
                                                      "z",
                                                      surface_mapping,
                                                      0.0,
                                                      key))
        .to eq(["Foo", nil])

      expect(TracklibRWGPS::section_column_simplified(reader,
                                                      0,
                                                      "F",
                                                      surface_mapping,
                                                      0.0,
                                                      key))
        .to eq(nil)
    end
  end
end
