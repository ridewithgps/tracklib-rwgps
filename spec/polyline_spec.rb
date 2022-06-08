require "spec_helper"

describe TracklibRWGPS do
  context "polylines" do
    it "can work in the simple case" do
      data = [{"x" => 40, "y" => 12, "e" => 2},
              {"x" => 41, "y" => 800, "e" => 2}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      polyline = TracklibRWGPS::section_data_polyline(reader, 0, TracklibRWGPS::PolylineOptions::new([["y", 5], ["x", 5]]))
      expect(decode_polyline(polyline, [5, 5]))
        .to eq([12.0, 40.0, 800.0, 41.0])

      # Parameters in the opposite order
      polyline = TracklibRWGPS::section_data_polyline(reader, 0, TracklibRWGPS::PolylineOptions::new([["x", 5], ["y", 5]]))
      expect(decode_polyline(polyline, [5, 5]))
        .to eq([40.0, 12.0, 41.0, 800.0])
    end

    it "can encode at different precisions" do
      data = [{"x" => 40, "y" => 12, "e" => 2},
              {"x" => 41, "y" => 800, "e" => 2}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      polyline = TracklibRWGPS::section_data_polyline(reader, 0, TracklibRWGPS::PolylineOptions::new([["y", 1], ["x", 5]]))
      expect(decode_polyline(polyline, [1, 5]))
        .to eq([12.0, 40.0, 800.0, 41.0])

      # Parameters in the opposite order
      polyline = TracklibRWGPS::section_data_polyline(reader, 0, TracklibRWGPS::PolylineOptions::new([["x", 5], ["y", 1]]))
      expect(decode_polyline(polyline, [5, 1]))
        .to eq([40.0, 12.0, 41.0, 800.0])
    end

    it "can skip points with a missing field" do
      data = [{"x" => 40, "y" => 12, "e" => 0.4},
              {"y" => 100, "e" => 0.5},
              {"x" => 41, "y" => 800, "e" => 0.6}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      polyline = TracklibRWGPS::section_data_polyline(reader, 0, TracklibRWGPS::PolylineOptions::new([["y", 5], ["x", 5]]))
      expect(decode_polyline(polyline, [5, 5]))
        .to eq([12.0, 40.0, 800.0, 41.0])
    end

    it "can handle default values" do
      data = [{"x" => 40, "y" => 12, "e" => 1, "S" => 10},
              {"x" => 100, "y" => 0.5, "e" => 1},
              {"x" => 41, "y" => 800, "e" => 1, "S" => 20}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7], ["S", :i64]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      polyline = TracklibRWGPS::section_data_polyline(reader, 0, TracklibRWGPS::PolylineOptions::new([["y", 5], ["x", 5], ["S", 5, 99]]))
      expect(decode_polyline(polyline, [5, 5, 5]))
        .to eq([12.0, 40.0, 10,
                0.5, 100, 99,
                800, 41, 20])
    end

    it "can skip encoding if a field is missing from the track" do
      data = [{"x" => 40, "y" => 12},
              {"x" => 41, "y" => 13}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      polyline = TracklibRWGPS::section_data_polyline(reader, 0, TracklibRWGPS::PolylineOptions::new([["y", 5], ["x", 5], ["e", 5]]))
      expect(polyline) .to eq("")

      polyline = TracklibRWGPS::section_data_polyline(reader, 0, TracklibRWGPS::PolylineOptions::new([["y", 5], ["x", 5], ["S", 5, 99]]))
      expect(polyline) .to eq("")
    end

    it "can encode encrypted sections" do
      data = [{"x" => 40, "y" => 12, "e" => 1},
              {"x" => 41, "y" => 800, "e" => 1}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7]])
      key = "01234567890123456789012345678901"
      section = Tracklib::Section::encrypted(schema, data, key)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      polyline = TracklibRWGPS::section_data_polyline(reader, 0, TracklibRWGPS::PolylineOptions::new([["y", 5], ["x", 5]]), key)
      expect(decode_polyline(polyline, [5, 5]))
        .to eq([12.0, 40.0, 800.0, 41.0])
    end

    it "can compute the 'd' field" do
      data = [{"x" => -122.402, "y" => 72.1, "e" => 2},
              {"x" => -122.500, "y" => 72.309, "e" => 2}]
      schema = Tracklib::Schema.new([["x", :f64, 7], ["y", :f64, 7], ["e", :f64, 7]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      polyline = TracklibRWGPS::section_data_polyline(reader, 0, TracklibRWGPS::PolylineOptions::new([["y", 5], ["x", 5], ["d", 5]]))
      expect(decode_polyline(polyline, [5, 5, 5]))
        .to eq([72.1, -122.402, 0.0, 72.309, -122.5, 23477.14945])
    end
  end
end
