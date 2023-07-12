require 'bundler/setup'
Bundler::setup()

require "tracklib-rwgps"
require "tracklib"

RSpec::configure do |config|
  RSpec::Expectations.configuration.on_potential_false_positives = :nothing
end

def decode_polyline(polyline, precisions)
  points = []
  index = 0
  prevs = precisions.collect{|_| 0.0}

  while index < polyline.length
    precisions.each_with_index do |precision, precision_index|
      result = 1
      shift = 0
      while true
        b = polyline[index].ord - 63 - 1
        index += 1
        result += b << shift
        shift += 5
        break if b < 0x1f
      end
      prevs[precision_index] += (result & 1) != 0 ? (~result >> 1) : (result >> 1)

      points << prevs[precision_index] / ( 10 ** precision )
    end

  end

  points
end
