require "spec_helper"

describe TracklibRwgps do
  context "simplify and encode" do
    it "can work in the simple case" do
      data = [{"x" => 40, "y" => 12, "e" => 2, "S" => 0, "R" => 0},
              {"x" => 41, "y" => 800, "e" => 2},
              {"x" => 42, "y" => 20000, "e" => 2, "S" => 10, "R" => 11}]
      schema = Tracklib::Schema.new([["x", :f64, 6], ["y", :f64, 6], ["e", :f64, 1], ["S", :u64], ["R", :u64]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      surface_mapping = TracklibRwgps::SurfaceMapping::new(99)

      polyline = TracklibRwgps::section_data_simplified_polyline(reader,
                                                                 0,
                                                                 surface_mapping,
                                                                 0.0,
                                                                 TracklibRwgps::PolylineOptions::new([["y", 5],
                                                                                                      ["x", 5],
                                                                                                      ["S", 5, 99],
                                                                                                      ["R", 5, 99]]))
      expect(decode_polyline(polyline, [5, 5, 5, 5]))
        .to eq([12, 40, 0, 0,
                800, 41, 99, 99,
                20000, 42, 10, 11])
    end

    it "can simplify and encode encrypted sections" do
      data = [{"x" => 40, "y" => 12, "e" => 1},
              {"x" => 41, "y" => 800, "e" => 1}]
      schema = Tracklib::Schema.new([["x", :f64, 6], ["y", :f64, 6], ["e", :f64, 1]])
      key = "01234567890123456789012345678901"
      section = Tracklib::Section::encrypted(schema, data, key)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      surface_mapping = TracklibRwgps::SurfaceMapping::new(99)

      polyline = TracklibRwgps::section_data_simplified_polyline(reader,
                                                                 0,
                                                                 surface_mapping,
                                                                 0.0,
                                                                 TracklibRwgps::PolylineOptions::new([["y", 5], ["x", 5]]),
                                                                 key)

      expect(decode_polyline(polyline, [5, 5]))
        .to eq([12.0, 40.0, 800.0, 41.0])
    end

    it "can handle when points are missing important fields" do
      data = [{"x" => 40, "y" => 12},
              {"x" => 41, "y" => 800, "e" => 2}]
      schema = Tracklib::Schema.new([["x", :f64, 6], ["y", :f64, 6], ["e", :f64, 1]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      surface_mapping = TracklibRwgps::SurfaceMapping::new(99)

      polyline = TracklibRwgps::section_data_simplified_polyline(reader,
                                                                 0,
                                                                 surface_mapping,
                                                                 0.0,
                                                                 TracklibRwgps::PolylineOptions::new([["y", 5], ["x", 5]]))
      expect(decode_polyline(polyline, [5, 5]))
        .to eq([800.0, 41.0])
    end
  end

  context "simplify and serialize" do
    let(:data) {
      # going uphill in river view cemetery
      [{"x"=> -122.66972, "y"=> 45.46463, "e"=> 35.7, "d"=>    0.0},
       {"x"=> -122.66984, "y"=> 45.46541, "e"=> 42.1, "d"=>   87.3},
       {},
       {},
       {},
       {"x"=> -122.66990, "y"=> 45.46554, "e"=> 42.4, "d"=>  102.5},
       {"x"=> -122.67005, "y"=> 45.46573, "e"=> 43.3, "d"=>  126.7},
       {"x"=> -122.67068, "y"=> 45.46616, "e"=> 53.5, "d"=>  195.3},
       {"x"=> -122.67078, "y"=> 45.46630, "e"=> 54.5, "d"=>  212.7},
       {"x"=> -122.67085, "y"=> 45.46648, "e"=> 55.0, "d"=>  233.5},
       {"x"=> -122.67091, "y"=> 45.46662, "e"=> 55.6, "d"=>  249.8},
       {"x"=> -122.67099, "y"=> 45.46716, "e"=> 54.5, "d"=>  310.2},
       {"x"=> -122.67108, "y"=> 45.46715, "e"=> 56.2, "d"=>  317.3},
       {"x"=> -122.67114, "y"=> 45.46710, "e"=> 57.0, "d"=>  324.6},
       {"x"=> -122.67122, "y"=> 45.46701, "e"=> 57.8, "d"=>  336.4},
       {"x"=> -122.67132, "y"=> 45.46678, "e"=> 58.7, "d"=>  363.2},
       {"x"=> -122.67144, "y"=> 45.46660, "e"=> 60.8, "d"=>  385.3},
       {"x"=> -122.67146, "y"=> 45.46646, "e"=> 62.2, "d"=>  401.0},
       {"x"=> -122.67134, "y"=> 45.46602, "e"=> 63.1, "d"=>  450.8},
       {"x"=> -122.67130, "y"=> 45.46577, "e"=> 64.7, "d"=>  478.8},
       {"x"=> -122.67118, "y"=> 45.46547, "e"=> 65.9, "d"=>  513.5},
       {"x"=> -122.67119, "y"=> 45.46538, "e"=> 66.6, "d"=>  523.6},
       {"x"=> -122.67128, "y"=> 45.46530, "e"=> 67.7, "d"=>  534.9},
       {"x"=> -122.67154, "y"=> 45.46522, "e"=> 70.4, "d"=>  557.1},
       {"x"=> -122.67166, "y"=> 45.46515, "e"=> 71.6, "d"=>  569.3},
       {"x"=> -122.67173, "y"=> 45.46505, "e"=> 72.0, "d"=>  581.7},
       {"x"=> -122.67177, "y"=> 45.46492, "e"=> 71.9, "d"=>  596.5},
       {"x"=> -122.67178, "y"=> 45.46480, "e"=> 71.5, "d"=>  609.8},
       {"x"=> -122.67175, "y"=> 45.46451, "e"=> 70.5, "d"=>  642.2},
       {"x"=> -122.67170, "y"=> 45.46431, "e"=> 70.1, "d"=>  664.8},
       {"x"=> -122.67149, "y"=> 45.46389, "e"=> 69.9, "d"=>  714.4},
       {"x"=> -122.67150, "y"=> 45.46378, "e"=> 70.9, "d"=>  726.6},
       {"x"=> -122.67160, "y"=> 45.46368, "e"=> 72.9, "d"=>  740.2},
       {"x"=> -122.67176, "y"=> 45.46361, "e"=> 75.3, "d"=>  755.0},
       {"x"=> -122.67201, "y"=> 45.46394, "e"=> 76.7, "d"=>  796.5},
       {"x"=> -122.67219, "y"=> 45.46413, "e"=> 77.7, "d"=>  821.9},
       {"x"=> -122.67240, "y"=> 45.46431, "e"=> 79.2, "d"=>  847.8},
       {"x"=> -122.67244, "y"=> 45.46428, "e"=> 79.9, "d"=>  852.4},
       {"x"=> -122.67248, "y"=> 45.46415, "e"=> 80.8, "d"=>  867.2},
       {"x"=> -122.67250, "y"=> 45.46385, "e"=> 81.5, "d"=>  900.6},
       {"x"=> -122.67246, "y"=> 45.46333, "e"=> 83.9, "d"=>  958.6},
       {"x"=> -122.67247, "y"=> 45.46314, "e"=> 84.4, "d"=>  979.8},
       {"x"=> -122.67255, "y"=> 45.46296, "e"=> 85.6, "d"=> 1000.8},
       {"x"=> -122.67264, "y"=> 45.46289, "e"=> 86.5, "d"=> 1011.3},
       {"x"=> -122.67275, "y"=> 45.46315, "e"=> 85.9, "d"=> 1041.4},
       {"x"=> -122.67291, "y"=> 45.46342, "e"=> 85.9, "d"=> 1074.0}]
    }

    let(:buf) {
      schema = Tracklib::Schema.new([["x", :f64, 6], ["y", :f64, 6], ["e", :f64, 1], ["d", :f64, 1]])
      section = Tracklib::Section::standard(schema, data)
      Tracklib::write_track([], [section])
    }

    let (:surface_mapping) { TracklibRwgps::SurfaceMapping::new(99) }

    it "works with no simplification" do
      reader = Tracklib::TrackReader::new(buf)

      expected = data.clone
      expected.delete_at(2) # remove the empties
      expected.delete_at(2)
      expected.delete_at(2)

      tolerance = 0.0

      expect(TracklibRwgps::section_data_simplified(reader,
                                                    0,
                                                    surface_mapping,
                                                    tolerance))
        .to eq(expected)
      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "x",
                                                      surface_mapping,
                                                      tolerance))
        .to eq(expected.map { |row| row["x"] })
    end

    it "works with a tolerance too small to simplify the track" do
      reader = Tracklib::TrackReader::new(buf)

      expected = data.clone
      expected.delete_at(2) # remove the empties
      expected.delete_at(2)
      expected.delete_at(2)

      tolerance = 0.000001

      expect(TracklibRwgps::section_data_simplified(reader,
                                                    0,
                                                    surface_mapping,
                                                    tolerance))
        .to eq(expected)
      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "x",
                                                      surface_mapping,
                                                      tolerance))
        .to eq(expected.map { |row| row["x"] })
    end

    it "works with a small simplification" do
      reader = Tracklib::TrackReader::new(buf)

      expected = data.clone
      expected.delete_at(2) # remove the empties
      expected.delete_at(2)
      expected.delete_at(2)
      expected.delete_at(6) # remove the points that get simplified away
      expected.delete_at(9)

      tolerance = 0.00001

      expect(TracklibRwgps::section_data_simplified(reader,
                                                    0,
                                                    surface_mapping,
                                                    tolerance))
        .to eq(expected)
      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "x",
                                                      surface_mapping,
                                                      tolerance))
        .to eq(expected.map { |row| row["x"] })
    end

    it "works with a big simplification" do
      reader = Tracklib::TrackReader::new(buf)

      expected = [data[0],
                  data[5],
                  data[7],
                  data[12],
                  data[17],
                  data[21],
                  data[24],
                  data[26],
                  data[31],
                  data[33],
                  data[36],
                  data[41],
                  data[43],
                  data[-1]]

      tolerance = 0.0001

      expect(TracklibRwgps::section_data_simplified(reader,
                                                    0,
                                                    surface_mapping,
                                                    tolerance))
        .to eq(expected)
      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "x",
                                                      surface_mapping,
                                                      tolerance))
        .to eq(expected.map { |row| row["x"] })
    end

    it "works with a radical simplification" do
      reader = Tracklib::TrackReader::new(buf)

      expected = [data[0],
                  data[-1]]

      tolerance = 0.1

      expect(TracklibRwgps::section_data_simplified(reader,
                                                    0,
                                                    surface_mapping,
                                                    tolerance))
        .to eq(expected)
      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "x",
                                                      surface_mapping,
                                                      tolerance))
        .to eq(expected.map { |row| row["x"] })
    end

    it "can work on encrypted sections" do
      schema = Tracklib::Schema.new([["x", :f64, 6], ["y", :f64, 6], ["e", :f64, 1], ["d", :f64, 1]])
      key = "01234567890123456789012345678901"
      section = Tracklib::Section::encrypted(schema, data, key)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      expected = data.clone
      expected.delete_at(2) # remove the empties
      expected.delete_at(2)
      expected.delete_at(2)
      expected.delete_at(6) # remove the points that get simplified away
      expected.delete_at(9)

      tolerance = 0.00001

      expect(TracklibRwgps::section_data_simplified(reader,
                                                    0,
                                                    surface_mapping,
                                                    tolerance,
                                                    key))
        .to eq(expected)
      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "x",
                                                      surface_mapping,
                                                      tolerance,
                                                      key))
        .to eq(expected.map { |row| row["x"] })
    end
  end

  context "simplify and serialize a single column" do
    it "can work in the simple case" do
      data = [{"x" => 40, "y" => 12, "e" => 2, "z" => "Foo"},
              {"x" => 41, "y" => 800, "e" => 2}]
      schema = Tracklib::Schema.new([["x", :f64, 6], ["y", :f64, 6], ["e", :f64, 1], ["z", :string]])
      section = Tracklib::Section::standard(schema, data)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      surface_mapping = TracklibRwgps::SurfaceMapping::new(99)

      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "x",
                                                      surface_mapping,
                                                      0.0))
        .to eq([40, 41])

      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "z",
                                                      surface_mapping,
                                                      0.0))
        .to eq(["Foo", nil])

      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "F",
                                                      surface_mapping,
                                                      0.0))
        .to eq(nil)
    end

    it "can work on encrypted sections" do
      data = [{"x" => 40, "y" => 12, "e" => 2, "z" => "Foo"},
              {"x" => 41, "y" => 800, "e" => 2}]
      schema = Tracklib::Schema.new([["x", :f64, 6], ["y", :f64, 6], ["e", :f64, 1], ["z", :string]])
      key = "01234567890123456789012345678901"
      section = Tracklib::Section::encrypted(schema, data, key)
      buf = Tracklib::write_track([], [section])
      reader = Tracklib::TrackReader::new(buf)

      surface_mapping = TracklibRwgps::SurfaceMapping::new(99)

      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "x",
                                                      surface_mapping,
                                                      0.0,
                                                      key))
        .to eq([40, 41])

      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "z",
                                                      surface_mapping,
                                                      0.0,
                                                      key))
        .to eq(["Foo", nil])

      expect(TracklibRwgps::section_column_simplified(reader,
                                                      0,
                                                      "F",
                                                      surface_mapping,
                                                      0.0,
                                                      key))
        .to eq(nil)
    end
  end
end
